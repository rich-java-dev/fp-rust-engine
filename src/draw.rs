
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d; // 1.4.0

pub fn clear_canvas(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_fill_style(&"black".into());
    ctx.fill_rect(0., 0., width, height);
}

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

    if elec_on {
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

pub fn draw_grid(ctx: &CanvasRenderingContext2d, width: f64, height: f64, color: &str) {
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
