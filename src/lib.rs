extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;

mod draw;
mod models;
mod phys;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
//     }

mod app {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
    use web_sys::CanvasRenderingContext2d; // 1.4.0

    use draw;
    use models;
    use phys::{
        calc_collision, calc_coloumb, calc_damping, calc_gravity, calc_wall_collision,
        collision_detected, recalc_position,
    };

    const PI: f64 = std::f64::consts::PI;

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
        }
    }

    #[wasm_bindgen]
    pub fn slow_down() {
        let mut p = PARAMS.lock().unwrap();

        for p1 in &mut p.coll {
            let p1: &mut models::Particle = p1;
            p1.vel_x = p1.vel_x * 0.8;
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
                for j in i..size {
                    if i == j {
                        continue;
                    }

                    let (set1, set2) = p.coll.split_at_mut(j);
                    let p1: &mut models::Particle = &mut set1[i];
                    let p2: &mut models::Particle = &mut set2[0];

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

                    if j == size - 1 {
                        if damping_on {
                            calc_damping(p1, d_val);
                        }
                        recalc_position(p1, c_val);
                        calc_wall_collision(p1, width, height);

                        // update last particle on the last pair-wise calculations
                        if j == i + 1 {
                            if damping_on {
                                calc_damping(p2, d_val);
                            }
                            recalc_position(p2, c_val);
                            calc_wall_collision(p2, width, height);
                        }
                    }
                }
            }
        }
        for p in &mut p.coll {
            let p1: &models::Particle = p;
            draw::draw_circle(
                ctx, elec_on, p1.pos_x, p1.pos_y, p1.radius, p1.charge, &p1.color,
            );
        }
    }
}
