extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;

use lazy_static::lazy_static; // 1.4.0
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }

#[derive(Serialize, Deserialize)]
pub struct Particle {
    mass: f64,
    radius: f64,
    pos_x: f64,
    pos_y: f64,
    vel_x: f64,
    vel_y: f64,
    charge: f64,
    color: String,
}

#[derive(Serialize, Deserialize)]
pub struct SysParams {
    coll: Vec<Particle>,
    c_on: bool,
    g_on: bool,
    e_on: bool,
    d_on: bool,
    grid_on: bool,
    c_val: f64,
    g_val: f64,
    k_val: f64,
    d_val: f64,
    width: f64,
    height: f64,
}

lazy_static! {
    static ref PARAMS: Mutex<SysParams> = Mutex::new(SysParams {
        coll: vec![],
        c_on: false,
        g_on: false,
        e_on: false,
        d_on: false,
        grid_on: false,
        c_val: 0.,
        g_val: 0.,
        k_val: 0.,
        d_val: 0.,
        width: 0.,
        height: 0.,
    });
}

#[wasm_bindgen]
pub fn set_params(params: &JsValue) {
    let state: SysParams = params.into_serde().unwrap();
    let mut p = PARAMS.lock().unwrap();

    p.coll = state.coll;
    p.c_on = state.c_on;
    p.g_on = state.g_on;
    p.e_on = state.e_on;
    p.d_on = state.d_on;
    p.grid_on = state.grid_on;
    p.c_val = state.c_val;
    p.g_val = state.g_val;
    p.k_val = state.k_val;
    p.d_val = state.d_val;
    p.width = state.width;
    p.height = state.height;
}

// pub fn get_params() -> JsValue {
//     let p = PARAMS.lock().unwrap();

//     let params = SysParams {
//         coll: p.coll,
//         c_on: p.c_on,
//         g_on: p.g_on,
//         e_on: p.e_on,
//         d_on: p.d_on,
//         c_val: p.c_val,
//         g_val: p.g_val,
//         k_val: p.k_val,
//         d_val: p.d_val,
//         width: p.width,
//         height: p.height,
//     };

//     JsValue::from_serde(&params).unwrap()
// }

#[wasm_bindgen]
pub fn step_system(ctx: &CanvasRenderingContext2d) {
    let mut p = PARAMS.lock().unwrap();

    clear_canvas(ctx, p.width, p.height);

    if p.grid_on {
        draw_grid(ctx, p.width, p.height, &"rgb(57, 255, 20)");
    }

    let e_on: bool = p.e_on;
    let g_on: bool = p.g_on;
    let c_on: bool = p.c_on;
    let d_on: bool = p.d_on;

    let g_val: f64 = p.g_val;
    let k_val: f64 = p.k_val;
    let d_val: f64 = p.d_val;
    let width: f64 = p.width;
    let height: f64 = p.height;

    let size = p.coll.len();
    for i in 0..size {
        for j in i..size {
            if i == j {
                continue;
            }

            let (set1, set2) = p.coll.split_at_mut(j);
            let p1: &mut Particle = &mut set1[i];
            let p2: &mut Particle = &mut set2[0];

            if g_on {
                calc_gravity(p1, p2, g_val);
            }
            if e_on {
                calc_coloumb(p1, p2, k_val);
            }
            if c_on {
                calc_collision(p1, p2);
            }

            if j == size - 1 {
                if d_on {
                    dampen(p1, d_val);
                }
                recalc_position(p1);
                calc_wall_collision(p1, width, height);

                draw_circle(
                    ctx, e_on, p1.pos_x, p1.pos_y, p1.radius, p1.charge, &p1.color,
                );
            }
        }
    }

    // for p1 in &mut collection {
    //     // iterate mutably
    //     let p1: &mut Particle = p1; // elements are mutable pointers
    //                                 // console_log!("pos_x:{}\npos_y:{}", p.pos_x, p.pos_y);

    //     recalc_position(&p1);
    //     draw_circle(
    //         ctx, state.e_on, p1.pos_x, p1.pos_y, p1.radius, p1.charge, &p1.color,
    //     );
    // }
}

fn dampen(e1: &mut Particle, d_val: f64) {
    e1.vel_x = e1.vel_x * (1. - d_val);
    e1.vel_y = e1.vel_y * (1. - d_val);
}

fn calc_collision(e1: &mut Particle, e2: &mut Particle) {
    let e1_m_ratio = (2. * e2.mass) / (e1.mass + e2.mass);
    let e2_m_ratio = (2. * e1.mass) / (e1.mass + e2.mass);

    let vecX_e12 = e1.pos_x - e2.pos_x;
    let vecY_e12 = e1.pos_y - e2.pos_y;
    let mut vec_xy_mag_sq = (e1.pos_x - e2.pos_x).powf(2.) + (e1.pos_y - e2.pos_y).powf(2.);
    let min_vec_xy_mag_sq = (e1.radius).powf(2.) + e2.radius.powf(2.);

    if vec_xy_mag_sq < min_vec_xy_mag_sq {
        vec_xy_mag_sq = min_vec_xy_mag_sq;
    }

    let e1_inner = (e1.pos_x - e2.pos_x) * (e1.vel_x - e2.vel_x)
        + (e1.pos_y - e2.pos_y) * (e1.vel_y - e2.vel_y);

    let e2_inner = (e2.pos_x - e1.pos_x) * (e2.vel_x - e1.vel_x)
        + (e2.pos_y - e1.pos_y) * (e2.vel_y - e1.vel_y);

    e1.vel_x = e1.vel_x - e1_m_ratio * (e1_inner / vec_xy_mag_sq) * vecX_e12;
    e1.vel_y = e1.vel_y - e1_m_ratio * (e1_inner / vec_xy_mag_sq) * vecY_e12;
    e2.vel_x = e2.vel_x + e2_m_ratio * (e2_inner / vec_xy_mag_sq) * vecX_e12;
    e2.vel_y = e2.vel_y + e2_m_ratio * (e2_inner / vec_xy_mag_sq) * vecY_e12;

    //prevent balls from getting 'stuck' together
    let rad_diff = e1.radius + e2.radius - (vecX_e12.powf(2.) + vecY_e12.powf(2.)).sqrt();

    let diff_comp = rad_diff / (1. as f64).powf(0.5);

    // e1 mass is larger, so should dominate and push e2
    if e1.mass > e2.mass {
        if vecX_e12 < 0. {
            e2.pos_x = e2.pos_x + diff_comp;
        } else {
            e2.pos_x = e2.pos_x - diff_comp;
        }

    //e2 mass is larger, so should dominate and push e1
    } else {
        if vecX_e12 < 0. {
            e1.pos_x = e1.pos_x - diff_comp;
        } else {
            e1.pos_x = e1.pos_x + diff_comp;
        }

        if vecY_e12 < 0. {
            e1.pos_y = e1.pos_y - diff_comp;
        } else {
            e1.pos_y = e1.pos_y + diff_comp;
        }
    }
}

fn calc_gravity(e1: &mut Particle, e2: &mut Particle, G: f64) {
    let vecX_e12: f64 = e1.pos_x - e2.pos_x;
    let vecY_e12: f64 = e1.pos_y - e2.pos_y;

    let dist_sq: f64 = vecX_e12.powf(2.) + vecY_e12.powf(2.);
    let dist: f64 = dist_sq.sqrt();
    let rx: f64 = vecX_e12 / dist;
    let ry: f64 = vecY_e12 / dist;
    e1.vel_x = e1.vel_x - G * (e2.mass / dist_sq) * rx;
    e1.vel_y = e1.vel_y - G * (e2.mass / dist_sq) * ry;
    e2.vel_x = e2.vel_x + G * (e1.mass / dist_sq) * rx;
    e2.vel_y = e2.vel_y + G * (e1.mass / dist_sq) * ry;
}

fn calc_coloumb(e1: &mut Particle, e2: &mut Particle, K: f64) {
    let vecX_e12: f64 = e1.pos_x - e2.pos_x;
    let vecY_e12: f64 = e1.pos_y - e2.pos_y;

    let dist_sq: f64 = vecX_e12.powf(2.) + vecY_e12.powf(2.);
    let dist: f64 = dist_sq.sqrt();
    let rx: f64 = vecX_e12 / dist;
    let ry: f64 = vecY_e12 / dist;

    let c_prod = e1.charge * e2.charge; // (qq term)

    e1.vel_x = e1.vel_x + K * (c_prod / (e1.mass * dist_sq)) * rx;
    e1.vel_y = e1.vel_y + K * (c_prod / (e1.mass * dist_sq)) * ry;

    e2.vel_x = e2.vel_x - K * (c_prod / (e2.mass * dist_sq)) * rx;
    e2.vel_y = e2.vel_y - K * (c_prod / (e2.mass * dist_sq)) * ry;
}

fn calc_wall_collision(e1: &mut Particle, width: f64, height: f64) {
    if e1.pos_y + e1.radius > height && e1.vel_y > 0. {
        e1.vel_y = -e1.vel_y.abs();
    } else if e1.pos_y - e1.radius < 0. && e1.vel_y < 0. {
        e1.vel_y = e1.vel_y.abs();
    }

    if e1.pos_x + e1.radius > width && e1.vel_x > 0. {
        e1.vel_x = -e1.vel_x.abs();
    } else if e1.pos_x - e1.radius < 0. && e1.vel_x < 0. {
        e1.vel_x = e1.vel_x.abs();
    }
}

fn clear_canvas(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_fill_style(&"black".into());
    ctx.fill_rect(0., 0., width, height);
}

fn recalc_position(p: &mut Particle) {
    // p.pos_x = p.pos_x + p.vel_x * 0.00;
    // p.pos_y = p.pos_y + p.vel_y * 0.0000000001;
}

fn draw_circle(
    ctx: &CanvasRenderingContext2d,
    e_on: bool,
    pos_x: f64,
    pos_y: f64,
    radius: f64,
    charge: f64,
    color: &String,
) {
    let color_json: JsValue = color.into();

    const PI: f64 = std::f64::consts::PI;
    ctx.begin_path();
    ctx.arc(pos_x, pos_y, radius + 2.0, 0.0, 2.0 * PI).unwrap();
    ctx.set_fill_style(&"black".into());
    ctx.fill();
    ctx.set_stroke_style(&"black".into());
    ctx.stroke();

    ctx.begin_path();
    ctx.arc(pos_x, pos_y, radius, 0.0, 2.0 * PI).unwrap();
    ctx.set_fill_style(&color_json);
    ctx.fill();
    ctx.set_stroke_style(&"black".into());
    ctx.stroke();

    if e_on {
        // geneates a plus/minus sign on the particles to indicate particle is charged
        let line_width: f64 = radius / 4.0;

        ctx.set_fill_style(&"black".into());
        ctx.fill_rect(
            pos_x - radius / 2.0,
            pos_y - line_width / 2.0,
            radius,
            line_width,
        );
        if charge > 0.0 {
            ctx.fill_rect(
                pos_x - line_width / 2.0,
                pos_y - radius / 2.0,
                line_width,
                radius,
            );
        }
    }
}

fn draw_grid(ctx: &CanvasRenderingContext2d, width: f64, height: f64, color: &str) {
    let color_json: JsValue = color.into();

    let spacing: f64 = 50.;

    let cols: f64 = width / spacing;
    let rows: f64 = height / spacing;

    for i in 1..(cols as i32 + 1) {
        ctx.begin_path();
        ctx.move_to((i as f64) * spacing, 0.);
        ctx.line_to((i as f64) * spacing, height);
        ctx.set_stroke_style(&color_json);
        ctx.stroke();
    }
    for i in 1..(rows as i32 + 1) {
        ctx.begin_path();
        ctx.move_to(0., (i as f64) * spacing);
        ctx.line_to(width, (i as f64) * spacing);
        ctx.set_stroke_style(&color_json);
        ctx.stroke();
    }
}

#[wasm_bindgen]
pub fn read_particle(particle: &JsValue) -> JsValue {
    let mut p: Particle = particle.into_serde().unwrap();

    p.pos_x = p.pos_x + p.vel_x;
    p.pos_y = p.pos_y + p.vel_y;

    // console_log!("pos_x:{}\npos_y:{}", p.pos_x, p.pos_y);

    JsValue::from_serde(&p).unwrap()
}
