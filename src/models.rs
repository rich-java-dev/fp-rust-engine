/**
 *
 * Basic structures to be transfered to/from JS/Rust.
 *
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Particle {
    pub mass: f64,
    pub radius: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub charge: f64,
    pub color: String,
    pub angle: f64,
    pub drag_detect: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SysParams {
    pub coll: Vec<Particle>,
    pub collisions_on: bool,
    pub gravity_on: bool,
    pub elec_on: bool,
    pub damping_on: bool,
    pub grid_on: bool,
    pub vectors_on: bool,
    pub c_val: f64,
    pub g_val: f64,
    pub k_val: f64,
    pub d_val: f64,
    pub width: f64,
    pub height: f64,
    pub is_paused: bool,
}
