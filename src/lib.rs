/**
 *
 * Basic "Controller"/central API for using in Web app.
 *
 */
extern crate lazy_static;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;

mod draw;
mod models;
mod phys;

mod app {

    /**
     * Imports Web Browser/JS "Alert" and "Log" functions for debugging/testing/etc.
     *
     */
    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);

        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
    use web_sys::CanvasRenderingContext2d;
    use web_sys::MouseEvent;

    use draw;
    use models;
    use phys::{
        calc_collision, calc_coloumb, calc_damping, calc_gravity, calc_wall_collision,
        collision_detected, recalc_position,
    };

    const PI: f64 = std::f64::consts::PI;

    /*
     *
     * Singleton storing 'global state' of the System objects/particles, and parameters
     *
     */
    lazy_static! {
        static ref PARAMS: Mutex<models::SysParams> = Mutex::new(models::SysParams {
            coll: vec![],
            collisions_on: false,
            gravity_on: false,
            elec_on: false,
            damping_on: false,
            grid_on: false,
            vectors_on: false,
            c_val: 0.,
            g_val: 0.,
            k_val: 0.,
            d_val: 0.,
            width: 0.,
            height: 0.,
            is_paused: false,
        });
    }

    /**
     *
     * Controller functions (wasm_bindgen)/external input/calls
     *
     */

    /*
     *
     * Set Params updates the entire app state
     *
     */
    #[wasm_bindgen]
    pub fn set_params(params: &JsValue) {
        let state: models::SysParams = params.into_serde().unwrap();
        let mut p = PARAMS.lock().unwrap();

        p.coll = state.coll;
        p.collisions_on = state.collisions_on;
        p.gravity_on = state.gravity_on;
        p.elec_on = state.elec_on;
        p.damping_on = state.damping_on;
        p.grid_on = state.grid_on;
        p.vectors_on = state.vectors_on;
        p.c_val = state.c_val;
        p.g_val = state.g_val;
        p.k_val = state.k_val;
        p.d_val = state.d_val;
        p.width = state.width;
        p.height = state.height;
        p.is_paused = state.is_paused;
    }

    #[wasm_bindgen]
    pub fn set_collection(collection: &JsValue) {
        let collect: Vec<models::Particle> = collection.into_serde().unwrap();
        let mut p = PARAMS.lock().unwrap();
        p.coll = collect;
    }

    #[wasm_bindgen]
    pub fn get_collection() -> JsValue {
        let params = PARAMS.lock().unwrap();
        JsValue::from_serde(&params.coll).unwrap()
    }

    #[wasm_bindgen]
    pub fn add_particle(particle: &JsValue) {
        let particle: models::Particle = particle.into_serde().unwrap();
        let mut p = PARAMS.lock().unwrap();
        p.coll.push(particle);
    }

    #[wasm_bindgen]
    pub fn remove_particle() {
        let mut p = PARAMS.lock().unwrap();
        p.coll.pop();
    }

    #[wasm_bindgen]
    pub fn enable_collisions(c_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.collisions_on = c_on;
    }

    #[wasm_bindgen]
    pub fn enable_pause(pause: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.is_paused = pause;
    }

    #[wasm_bindgen]
    pub fn enable_gravity(g_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.gravity_on = g_on;
    }

    #[wasm_bindgen]
    pub fn enable_electrostatics(e_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.elec_on = e_on;
    }

    #[wasm_bindgen]
    pub fn enable_damping(d_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.damping_on = d_on;
    }

    #[wasm_bindgen]
    pub fn enable_grid(grid_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.grid_on = grid_on;
    }

    #[wasm_bindgen]
    pub fn enable_vectors(vec_on: bool) {
        let mut p = PARAMS.lock().unwrap();
        p.vectors_on = vec_on;
    }

    #[wasm_bindgen]
    pub fn set_c_val(c_val: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.c_val = c_val;
    }

    #[wasm_bindgen]
    pub fn set_g_val(g_val: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.g_val = g_val;
    }

    #[wasm_bindgen]
    pub fn set_k_val(k_val: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.k_val = k_val;
    }

    #[wasm_bindgen]
    pub fn set_d_val(d_val: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.d_val = d_val;
    }

    #[wasm_bindgen]
    pub fn set_width(width: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.width = width;
    }

    #[wasm_bindgen]
    pub fn set_height(height: f64) {
        let mut p = PARAMS.lock().unwrap();
        p.height = height;
    }

    #[wasm_bindgen]
    pub fn speed_up() {
        let mut p = PARAMS.lock().unwrap();

        for p1 in &mut p.coll {
            let p1: &mut models::Particle = p1;
            p1.vel_x = p1.vel_x * 1.2;
            p1.vel_y = p1.vel_y * 1.2;
        }
    }

    #[wasm_bindgen]
    pub fn slow_down() {
        let mut p = PARAMS.lock().unwrap();

        for p1 in &mut p.coll {
            let p1: &mut models::Particle = p1;
            p1.vel_x = p1.vel_x * 0.8;
            p1.vel_y = p1.vel_y * 0.8;
        }
    }

    #[wasm_bindgen]
    pub fn expand_all() {
        let mut p = PARAMS.lock().unwrap();

        for p1 in &mut p.coll {
            let p1: &mut models::Particle = p1;
            p1.radius *= 1.2;
            p1.mass = PI * p1.radius.powf(2.);
        }
    }

    #[wasm_bindgen]
    pub fn shrink_all() {
        let mut p = PARAMS.lock().unwrap();

        for p1 in &mut p.coll {
            let p1: &mut models::Particle = p1;
            p1.radius *= 0.7;
            p1.mass = PI * p1.radius.powf(2.);
        }
    }

    #[wasm_bindgen]
    pub fn get_params() -> JsValue {
        let p = PARAMS.lock().unwrap();

        let params = models::SysParams {
            coll: p.coll.clone(),
            collisions_on: p.collisions_on,
            gravity_on: p.gravity_on,
            elec_on: p.elec_on,
            damping_on: p.damping_on,
            grid_on: p.grid_on,
            vectors_on: p.vectors_on,
            c_val: p.c_val,
            g_val: p.g_val,
            k_val: p.k_val,
            d_val: p.d_val,
            width: p.width,
            height: p.height,
            is_paused: p.is_paused,
        };

        JsValue::from_serde(&params).unwrap()
    }

    #[wasm_bindgen]
    pub fn step_system(ctx: &CanvasRenderingContext2d) {
        let mut p = PARAMS.lock().unwrap();

        draw::clear_canvas(ctx, p.width, p.height);

        if p.grid_on {
            draw::draw_grid(ctx, p.width, p.height, &"rgb(57, 255, 20)");
        }

        let elec_on: bool = p.elec_on;
        let gravity_on: bool = p.gravity_on;
        let collisions_on: bool = p.collisions_on;
        let damping_on: bool = p.damping_on;
        let is_paused: bool = p.is_paused;

        let g_val: f64 = p.g_val;
        let k_val: f64 = p.k_val;
        let d_val: f64 = p.d_val;
        let c_val: f64 = p.c_val;
        let width: f64 = p.width;
        let height: f64 = p.height;

        if !is_paused {
            let size = p.coll.len();

            for i in 0..size {
                let (p1, rest) = p.coll[i..].split_first_mut().unwrap();
                for p2 in rest {
                    if gravity_on {
                        calc_gravity(p1, p2, g_val);
                    }
                    if elec_on {
                        calc_coloumb(p1, p2, k_val);
                    }
                    if collisions_on {
                        if collision_detected(p1, p2) {
                            calc_collision(p1, p2);
                        }
                    }
                }
            }
            for body in p.coll.iter_mut() {
                if damping_on {
                    calc_damping(body, d_val);
                }
                recalc_position(body, c_val);
                calc_wall_collision(body, width, height);
            }
        }
        if p.vectors_on {
            draw::draw_vectors(ctx, &mut p);
        }
        for body in p.coll.iter_mut() {
            draw::draw_circle(
                ctx,
                elec_on,
                body.pos_x,
                body.pos_y,
                body.radius,
                body.charge,
                &body.color,
            );
        }
    }

    #[wasm_bindgen]
    pub fn detect_drag(x: f64, y: f64) {
        let mut p = PARAMS.lock().unwrap();

        for body in p.coll.iter_mut() {
            if body.pos_x + body.radius > x && body.pos_x - body.radius < x {
                if body.pos_y + body.radius > y && body.pos_y - body.radius < y {
                    body.drag_detect = true;
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn handle_drag(x: f64, y: f64) {
        let mut p = PARAMS.lock().unwrap();
        for body in p.coll.iter_mut() {
            if body.drag_detect {
                body.pos_x = x;
                body.pos_y = y;
                body.vel_x = 0.;
                body.vel_y = 0.;
            }
        }
    }

    #[wasm_bindgen]
    pub fn end_drag() {
        let mut p = PARAMS.lock().unwrap();

        for body in p.coll.iter_mut() {
            if body.drag_detect {
                body.drag_detect = false;
            }
        }
    }
}
