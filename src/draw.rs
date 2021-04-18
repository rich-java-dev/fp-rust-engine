/**
 *
 * HTML5 Canvas App API for Rust/WASM mapping calls.
 *
 *
 */
extern crate js_sys;

use models;
use phys::{calc_coloumb, calc_gravity};
use wasm_bindgen::prelude::JsValue;
use web_sys::CanvasRenderingContext2d;

const PI: f64 = std::f64::consts::PI;

pub fn clear_canvas(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_fill_style(&"black".into());
    ctx.fill_rect(0., 0., width, height);
}

/**
 *
 * Renders a basic circle in the given position on canvas
 */
pub fn draw_circle(
    ctx: &CanvasRenderingContext2d,
    elec_on: bool,
    pos_x: f64,
    pos_y: f64,
    radius: f64,
    charge: f64,
    color: &String,
) {
    let color_json: JsValue = color.into();

    ctx.begin_path();
    ctx.arc(pos_x, pos_y, radius + 2., 0., 2. * PI).unwrap();
    ctx.set_fill_style(&"black".into());
    ctx.fill();
    ctx.set_stroke_style(&"black".into());
    ctx.stroke();

    ctx.begin_path();
    ctx.arc(pos_x, pos_y, radius + 1., 0., 2. * PI).unwrap();
    ctx.set_fill_style(&color_json);
    ctx.fill();
    ctx.set_stroke_style(&"black".into());
    ctx.stroke();

    if elec_on {
        // generates a plus/minus sign on the particles to indicate particle is charged
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

pub fn draw_grid(ctx: &CanvasRenderingContext2d, width: f64, height: f64, color: &str) {
    let color_json: JsValue = color.into();

    let spacing: f64 = 50.;

    let cols: f64 = width / spacing;
    let rows: f64 = height / spacing;

    for i in 1..(cols as i32 + 1) {
        ctx.set_line_width(0.5);
        ctx.begin_path();
        ctx.move_to((i as f64) * spacing, 0.);
        ctx.line_to((i as f64) * spacing, height);
        ctx.set_stroke_style(&color_json);
        ctx.stroke();
    }
    for i in 1..(rows as i32 + 1) {
        ctx.set_line_width(0.5);
        ctx.begin_path();
        ctx.move_to(0., (i as f64) * spacing);
        ctx.line_to(width, (i as f64) * spacing);
        ctx.set_stroke_style(&color_json);
        ctx.stroke();
    }
}

pub fn draw_vectors(ctx: &CanvasRenderingContext2d, params: &mut models::SysParams) {
    let spacing: f64 = 50.;

    let cols: f64 = params.width / spacing;
    let rows: f64 = params.height / spacing;

    for c in 0..(cols as i32 + 1) * 3 {
        for r in 0..(rows as i32 + 1) * 3 {
            let mut test_pt: models::Particle = models::Particle {
                mass: 1e-4,
                radius: 7.,
                pos_x: ((c as f64) * spacing) / 3.,
                pos_y: ((r as f64) * spacing) / 3.,
                vel_x: 0.,
                vel_y: 0.,
                charge: 1e-4,
                color: String::from("rgb(57, 255, 20)"),
                angle: 0.,
                drag_detect: false,
            };

            for body in params.coll.iter_mut() {
                if params.gravity_on {
                    calc_gravity(body, &mut test_pt, params.g_val);
                }
                if params.elec_on {
                    calc_coloumb(body, &mut test_pt, params.k_val);
                }
            }
            let angle = calc_atan(&mut test_pt);
            test_pt.angle = angle;
            draw_arrow(ctx, &mut test_pt);
        }
    }
}

fn calc_atan(p: &models::Particle) -> f64 {
    js_sys::Math::atan2(p.vel_y, p.vel_x)
}

pub fn draw_arrow(ctx: &CanvasRenderingContext2d, p: &mut models::Particle) {
    let p_x = p.pos_x + p.radius * p.angle.cos();
    let p_y = p.pos_y + p.radius * p.angle.sin();

    ctx.set_stroke_style(&"red".into());
    ctx.set_line_width(0.5);
    ctx.begin_path();
    ctx.move_to(p.pos_x, p.pos_y);
    ctx.line_to(p_x, p_y);
    ctx.stroke();
}
