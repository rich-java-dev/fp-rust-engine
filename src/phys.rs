/**
 *
 * 2D physics lib which operates on input data (Particles/Pairs)
 * and modifies their underlying values based on the interaction specified.
 *
 */
use models::Particle;

/**
 *
 * System designed such that time steps are normalized (unit 'vector')
 * This means the time evolution is represented as:
 *
 * dX = VdT
 * Therefore Xn = X(n-1) + V(x)
 *
 * Where V (Velocity function can be a non-trivial function of X)
 *
 * In this case, V is quadratic under Inverse Square Laws of Gravity and Coulomb Force:
 * ((Gmm || Kqq  )/r^2) where r is the distance between 2 points
 *
 */
pub fn recalc_position(p: &mut Particle, max_speed: f64) {
    if (p.vel_x.powf(2.) + p.vel_y.powf(2.)).powf(0.5) > max_speed {
        p.vel_x = p.vel_x / max_speed;
        p.vel_y = p.vel_y / max_speed;
    }
    p.pos_x = p.pos_x + p.vel_x;
    p.pos_y = p.pos_y + p.vel_y;
}

/**
 *
 * Exponential Damping function (In which over time,
 * the Velocity is reduced by some percentage of its current velocity
 * Note: This is also usable as a Driving function, of negative inputs)
 */
pub fn calc_damping(e1: &mut Particle, d_val: f64) {
    e1.vel_x = e1.vel_x * (1. - d_val);
    e1.vel_y = e1.vel_y * (1. - d_val);
}

/**
 *
 * Basic 'hit box' detection
 *
 */
pub fn collision_detected(e1: &mut Particle, e2: &mut Particle) -> bool {
    let diff_x = e1.pos_x - e2.pos_x;
    let diff_y = e1.pos_y - e2.pos_y;
    let rad_diff = (diff_x.powf(2.) + diff_y.powf(2.)).powf(0.5);
    rad_diff <= e1.radius + e2.radius + 1.
}

/**
 *
 * Elastic Collision:
 *
 * for every pair of interacting particles, pre/post interaction state must preserve these properties.
 *
 * conservation of Momentum (P) (p = mv)
 * conversation of Kinetic Energy (KE): .5mv^2 => (p^2)/2m
 * convervation of Mass.. (Not Nuclear... eg: E = mc^2)
 *
 * eg:
 *
 * (P1 Moment Initial + P2 Moment Initial = P1 Moment Final + P2 Moment Final)
 *  m1v1i + m2v2i = m1v1f + m2v2f
 *
 * (KE P1 Initial + KE P2 Initial = KE P1 Final + KE P2 Final)
 *  m1v1i^2 + m2v2i^2 = m1v1f^2 + m2v2f^2
 *
 * solving for v1f, v2f in terms of Initials values (m1, m2, v1i, v2i)
 *  
 *
 */
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

    // Determining the "Dominant" Role in the Interaction,
    // by which the Systems derives which particle is 'Pushed'
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

/**
 *
 * Newtonian Gravity:
 *
 * F = Gm1m2/r^2
 *
 * Where r is the distance between the 2 bodies centers.
 *
 * in 2d:  r = sqrt(dx^2 + dy^2)
 * in 3d:  r = sqrt(dx^2 + dy^x + dz^2)
 * (Pythagoreon theorem of (a^2 + b^2 = c^2); c => sqrt(a^2 + b^2))
 *
 *
 * Newton's 2nd Law:
 *
 * F = ma
 *
 * a = v/t
 *
 * p = mv
 *
 * F = mv/t (p/t)
 *
 * F = Gmm/r^2 = mv
 *
 * where r^2 = (dX^2 + dY^x)
 *  
 * F = ma
 *  
 * F = mv/t
 *
 * F1 = Gm1m2/r^2 = m1v1 (Force felt on particle 1 by particle 2)
 *
 * dv1 = Gm2/r^2
 *
 *
 */
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

/**
 *
 * Coloumb Force:
 *
 * F = Kq1q2/r^2
 *
 * Where r is the distance between the 2 bodies centers.
 *
 * in 2d:  r = sqrt(dx^2 + dy^2)
 * in 3d:  r = sqrt(dx^2 + dy^x + dz^2)
 * (Pythagoreon theorem: (a^2 + b^2 = c^2); c => sqrt(a^2 + b^2))
 *
 *
 * Newton's 2nd Law:
 *
 * F = ma
 *
 * a = v/t
 *
 * p = mv
 *
 * F = mv/t (p/t)
 *
 * F = Gmm/r^2 = mv
 *
 * where r^2 = (dX^2 + dY^x)
 *  
 * F = ma
 *  
 * F = mv/t
 *
 * F1 = Gm1m2/r^2 = m1v1 (Force felt on particle 1 by particle 2)
 *
 * dv1 = Gm2/r^2
 *
 *
 */
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

/**
 *
 * Particles are contained inside of the Canvas like particles in 'box'.
 * Follows basic reflection for wall collision.
 *
 * For large numbers of particles in a confined spaced, behaves like a Compress Gas/Pressure build-up
 *
 *
 */
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
