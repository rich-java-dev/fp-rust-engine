use models::Particle;

pub fn calc_damping(e1: &mut Particle, d_val: f64) {
    e1.vel_x = e1.vel_x * (1. - d_val);
    e1.vel_y = e1.vel_y * (1. - d_val);
}
pub fn collision_detected(e1: &mut Particle, e2: &mut Particle) -> bool {
    let diff_x = e1.pos_x - e2.pos_x;
    let diff_y = e1.pos_y - e2.pos_y;
    let rad_diff = (diff_x.powf(2.) + diff_y.powf(2.)).powf(0.5);
    rad_diff <= e1.radius + e2.radius
}
pub fn calc_collision(e1: &mut Particle, e2: &mut Particle) {
    let e1_m_ratio = (2. * e2.mass) / (e1.mass + e2.mass);
    let e2_m_ratio = (2. * e1.mass) / (e1.mass + e2.mass);
    let vec_x_e12 = e1.pos_x - e2.pos_x;
    let vec_y_e12 = e1.pos_y - e2.pos_y;
    let mut vec_xy_mag_sq = (e1.pos_x - e2.pos_x).powf(2.) + (e1.pos_y - e2.pos_y).powf(2.);
    let min_vec_xy_mag_sq = (e1.radius).powf(2.) + e2.radius.powf(2.);
    if vec_xy_mag_sq < min_vec_xy_mag_sq {
        vec_xy_mag_sq = min_vec_xy_mag_sq;
    }
    let e1_inner = (e1.pos_x - e2.pos_x) * (e1.vel_x - e2.vel_x)
        + (e1.pos_y - e2.pos_y) * (e1.vel_y - e2.vel_y);
    let e2_inner = (e2.pos_x - e1.pos_x) * (e2.vel_x - e1.vel_x)
        + (e2.pos_y - e1.pos_y) * (e2.vel_y - e1.vel_y);
    e1.vel_x = e1.vel_x - e1_m_ratio * (e1_inner / vec_xy_mag_sq) * vec_x_e12;
    e1.vel_y = e1.vel_y - e1_m_ratio * (e1_inner / vec_xy_mag_sq) * vec_y_e12;
    e2.vel_x = e2.vel_x + e2_m_ratio * (e2_inner / vec_xy_mag_sq) * vec_x_e12;
    e2.vel_y = e2.vel_y + e2_m_ratio * (e2_inner / vec_xy_mag_sq) * vec_y_e12;
    //prevent balls from getting 'stuck' together
    let rad_diff = e1.radius + e2.radius - (vec_x_e12.powf(2.) + vec_y_e12.powf(2.)).sqrt();
    let diff_comp = rad_diff / (2. as f64).powf(0.5);
    // e1 mass is larger, so should dominate and push e2
    if e1.mass > e2.mass {
        if vec_x_e12 < 0. {
            e2.pos_x = e2.pos_x + diff_comp;
        } else {
            e2.pos_x = e2.pos_x - diff_comp;
        }
    //e2 mass is larger, so should dominate and push e1
    } else {
        if vec_x_e12 < 0. {
            e1.pos_x = e1.pos_x - diff_comp;
        } else {
            e1.pos_x = e1.pos_x + diff_comp;
        }
        if vec_y_e12 < 0. {
            e1.pos_y = e1.pos_y - diff_comp;
        } else {
            e1.pos_y = e1.pos_y + diff_comp;
        }
    }
}
pub fn calc_gravity(e1: &mut Particle, e2: &mut Particle, G: f64) {
    let vec_x_e12: f64 = e1.pos_x - e2.pos_x;
    let vec_y_e12: f64 = e1.pos_y - e2.pos_y;
    let dist_sq: f64 = vec_x_e12.powf(2.) + vec_y_e12.powf(2.);
    let dist: f64 = dist_sq.sqrt();
    let rx: f64 = vec_x_e12 / dist;
    let ry: f64 = vec_y_e12 / dist;
    e1.vel_x = e1.vel_x - G * (e2.mass / dist_sq) * rx;
    e1.vel_y = e1.vel_y - G * (e2.mass / dist_sq) * ry;
    e2.vel_x = e2.vel_x + G * (e1.mass / dist_sq) * rx;
    e2.vel_y = e2.vel_y + G * (e1.mass / dist_sq) * ry;
}
pub fn calc_coloumb(e1: &mut Particle, e2: &mut Particle, K: f64) {
    let vec_x_e12: f64 = e1.pos_x - e2.pos_x;
    let vec_y_e12: f64 = e1.pos_y - e2.pos_y;
    let dist_sq: f64 = vec_x_e12.powf(2.) + vec_y_e12.powf(2.);
    let dist: f64 = dist_sq.sqrt();
    let rx: f64 = vec_x_e12 / dist;
    let ry: f64 = vec_y_e12 / dist;
    let c_prod = e1.charge * e2.charge; // (qq term)
    e1.vel_x = e1.vel_x + K * (c_prod / (e1.mass * dist_sq)) * rx;
    e1.vel_y = e1.vel_y + K * (c_prod / (e1.mass * dist_sq)) * ry;
    e2.vel_x = e2.vel_x - K * (c_prod / (e2.mass * dist_sq)) * rx;
    e2.vel_y = e2.vel_y - K * (c_prod / (e2.mass * dist_sq)) * ry;
}
pub fn calc_wall_collision(e1: &mut Particle, width: f64, height: f64) {
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

pub fn recalc_position(p: &mut Particle, max_speed: f64) {
    if (p.vel_x.powf(2.) + p.vel_y.powf(2.)).powf(0.5) > max_speed {
        p.vel_x = p.vel_x / max_speed;
        p.vel_y = p.vel_y / max_speed;
    }
    p.pos_x = p.pos_x + p.vel_x;
    p.pos_y = p.pos_y + p.vel_y;
}
