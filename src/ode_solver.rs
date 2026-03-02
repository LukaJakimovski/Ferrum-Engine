use glam::{DVec2};
pub fn rk4_gravity_step(
    _t: f64,
    x: DVec2,
    v: DVec2,
    dt: f64,
    _m: f64,
    // The closure now takes (current_position, current_velocity, time_offset)
    // so it can calculate the "world" at k1, k2, k3, k4
    accel_fn: &dyn Fn(DVec2, DVec2, f64) -> DVec2,
) -> (DVec2, DVec2) {
    let k1_v = v;
    let k1_a = accel_fn(x, v, 0.0);

    let k2_v = v + k1_a * (dt * 0.5);
    let k2_a = accel_fn(x + k1_v * (dt * 0.5), v + k1_a * (dt * 0.5), dt * 0.5);

    let k3_v = v + k2_a * (dt * 0.5);
    let k3_a = accel_fn(x + k2_v * (dt * 0.5), v + k2_a * (dt * 0.5), dt * 0.5);

    let k4_v = v + k3_a * dt;
    let k4_a = accel_fn(x + k3_v * dt, v + k3_a * dt, dt);

    let new_x = x + (k1_v + k2_v * 2.0 + k3_v * 2.0 + k4_v) * (dt / 6.0);
    let new_v = v + (k1_a + k2_a * 2.0 + k3_a * 2.0 + k4_a) * (dt / 6.0);

    (new_x, new_v)
}




pub fn rk4_step(
    t: f64,
    x: DVec2,
    v: DVec2,
    dt: f64,
    m: f64,
    force: &dyn Fn(f64, DVec2, DVec2) -> DVec2,
) -> (DVec2, DVec2) {
    let a = |t: f64, x: DVec2, v: DVec2| force(t, x, v) / m;

    let k1_x = v;
    let k1_v = a(t, x, v);

    let k2_x = v + k1_v * (0.5 * dt);
    let k2_v = a(t + 0.5 * dt, x + k1_x * (0.5 * dt), v + k1_v * (0.5 * dt));

    let k3_x = v + k2_v * (0.5 * dt);
    let k3_v = a(t + 0.5 * dt, x + k2_x * (0.5 * dt), v + k2_v * (0.5 * dt));

    let k4_x = v + k3_v * dt;
    let k4_v = a(t + dt, x + k3_x * dt, v + k3_v * dt);

    let x_next = x + (k1_x + k2_x * 2.0 + k3_x * 2.0 + k4_x) * (dt / 6.0);
    let v_next = v + (k1_v + k2_v * 2.0 + k3_v * 2.0 + k4_v) * (dt / 6.0);

    (x_next, v_next)
}

pub fn rk4_angular_step(
    t: f64,
    angle: f64,
    angular_velocity: f64,
    dt: f64,
    moment_of_inertia: f64,
    torque: &dyn Fn(f64, f64, f64) -> f64,
) -> (f64, f64) {
    let alpha = |t: f64, theta: f64, omega: f64| torque(t, theta, omega) / moment_of_inertia;

    let k1_theta = angular_velocity;
    let k1_omega = alpha(t, angle, angular_velocity);

    let k2_theta = angular_velocity + 0.5 * dt * k1_omega;
    let k2_omega = alpha(
        t + 0.5 * dt,
        angle + 0.5 * dt * k1_theta,
        angular_velocity + 0.5 * dt * k1_omega,
    );

    let k3_theta = angular_velocity + 0.5 * dt * k2_omega;
    let k3_omega = alpha(
        t + 0.5 * dt,
        angle + 0.5 * dt * k2_theta,
        angular_velocity + 0.5 * dt * k2_omega,
    );

    let k4_theta = angular_velocity + dt * k3_omega;
    let k4_omega = alpha(
        t + dt,
        angle + dt * k3_theta,
        angular_velocity + dt * k3_omega,
    );

    let theta_next = angle + dt / 6.0 * (k1_theta + 2.0 * k2_theta + 2.0 * k3_theta + k4_theta);
    let omega_next =
        angular_velocity + dt / 6.0 * (k1_omega + 2.0 * k2_omega + 2.0 * k3_omega + k4_omega);

    (theta_next, omega_next)
}

pub fn dormand_prince_step(
    t: f64,
    x: DVec2,
    v: DVec2,
    dt: f64,
    m: f64,
    force: &dyn Fn(f64, DVec2, DVec2) -> DVec2,
) -> (DVec2, DVec2){ // ((DVec2, DVec2), (DVec2, DVec2)) {
    let a = |t: f64, x: DVec2, v: DVec2| force(t, x, v) / m;

    // k1
    let k1_x = v;
    let k1_v = a(t, x, v);

    // k2
    let k2_x = v + k1_v * (dt * (1.0 / 5.0));
    let k2_v = a(t + dt * (1.0 / 5.0), x + k1_x * (dt * (1.0 / 5.0)), v + k1_v * (dt * (1.0 / 5.0)));

    // k3
    let k3_x = v + k1_v * (dt * (3.0 / 40.0)) + k2_v * (dt * (9.0 / 40.0));
    let k3_v = a(
        t + dt * (3.0 / 10.0),
        x + k1_x * (dt * (3.0 / 40.0)) + k2_x * (dt * (9.0 / 40.0)),
        v + k1_v * (dt * (3.0 / 40.0)) + k2_v * (dt * (9.0 / 40.0)),
    );

    // k4
    let k4_x = v
        + k1_v * (dt * (44.0 / 45.0))
        + k2_v * (dt * (-56.0 / 15.0))
        + k3_v * (dt * (32.0 / 9.0));
    let k4_v = a(
        t + dt * (4.0 / 5.0),
        x + k1_x * (dt * (44.0 / 45.0)) + k2_x * (dt * (-56.0 / 15.0)) + k3_x * (dt * (32.0 / 9.0)),
        v + k1_v * (dt * (44.0 / 45.0)) + k2_v * (dt * (-56.0 / 15.0)) + k3_v * (dt * (32.0 / 9.0)),
    );

    // k5
    let k5_x = v
        + k1_v * (dt * (19372.0 / 6561.0))
        + k2_v * (dt * (-25360.0 / 2187.0))
        + k3_v * (dt * (64448.0 / 6561.0))
        + k4_v * (dt * (-212.0 / 729.0));
    let k5_v = a(
        t + dt * (8.0 / 9.0),
        x + k1_x * (dt * (19372.0 / 6561.0))
            + k2_x * (dt * (-25360.0 / 2187.0))
            + k3_x * (dt * (64448.0 / 6561.0))
            + k4_x * (dt * (-212.0 / 729.0)),
        v + k1_v * (dt * (19372.0 / 6561.0))
            + k2_v * (dt * (-25360.0 / 2187.0))
            + k3_v * (dt * (64448.0 / 6561.0))
            + k4_v * (dt * (-212.0 / 729.0)),
    );

    // k6
    let k6_x = v
        + k1_v * (dt * (9017.0 / 3168.0))
        + k2_v * (dt * (-355.0 / 33.0))
        + k3_v * (dt * (46732.0 / 5247.0))
        + k4_v * (dt * (49.0 / 176.0))
        + k5_v * (dt * (-5103.0 / 18656.0));
    let k6_v = a(
        t + dt,
        x + k1_x * (dt * (9017.0 / 3168.0))
            + k2_x * (dt * (-355.0 / 33.0))
            + k3_x * (dt * (46732.0 / 5247.0))
            + k4_x * (dt * (49.0 / 176.0))
            + k5_x * (dt * (-5103.0 / 18656.0)),
        v + k1_v * (dt * (9017.0 / 3168.0))
            + k2_v * (dt * (-355.0 / 33.0))
            + k3_v * (dt * (46732.0 / 5247.0))
            + k4_v * (dt * (49.0 / 176.0))
            + k5_v * (dt * (-5103.0 / 18656.0)),
    );

    // k7 (final)
    let k7_x = v
        + k1_v * (dt * (35.0 / 384.0))
        + k3_v * (dt * (500.0 / 1113.0))
        + k4_v * (dt * (125.0 / 192.0))
        + k5_v * (dt * (-2187.0 / 6784.0))
        + k6_v * (dt * (11.0 / 84.0));
    let k7_v = a(
        t + dt,
        x + k1_x * (dt * (35.0 / 384.0))
            + k3_x * (dt * (500.0 / 1113.0))
            + k4_x * (dt * (125.0 / 192.0))
            + k5_x * (dt * (-2187.0 / 6784.0))
            + k6_x * (dt * (11.0 / 84.0)),
        v + k1_v * (dt * (35.0 / 384.0))
            + k3_v * (dt * (500.0 / 1113.0))
            + k4_v * (dt * (125.0 / 192.0))
            + k5_v * (dt * (-2187.0 / 6784.0))
            + k6_v * (dt * (11.0 / 84.0)),
    );

    // Compute 5th order estimate for next step
    let x_next = x
        + dt * (k1_x * (35.0 / 384.0)
        + k3_x * (500.0 / 1113.0)
        + k4_x * (125.0 / 192.0)
        + k5_x * (-2187.0 / 6784.0)
        + k6_x * (11.0 / 84.0));
    let v_next = v
        + dt * (k1_v * (35.0 / 384.0)
        + k3_v * (500.0 / 1113.0)
        + k4_v * (125.0 / 192.0)
        + k5_v * (-2187.0 / 6784.0)
        + k6_v * (11.0 / 84.0));

    // 4th order estimate (for error estimate)
    let _x_star = x
        + dt * (k1_x * (5179.0 / 57600.0)
        + k3_x * (7571.0 / 16695.0)
        + k4_x * (393.0 / 640.0)
        + k5_x * (-92097.0 / 339200.0)
        + k6_x * (187.0 / 2100.0)
        + k7_x * (1.0 / 40.0));
    let _v_star = v
        + dt * (k1_v * (5179.0 / 57600.0)
        + k3_v * (7571.0 / 16695.0)
        + k4_v * (393.0 / 640.0)
        + k5_v * (-92097.0 / 339200.0)
        + k6_v * (187.0 / 2100.0)
        + k7_v * (1.0 / 40.0));

    //((x_next, v_next), (x_next - x_star, v_next - v_star))
    (x_next, v_next)
}


pub fn rkdp_step(
    x: DVec2,
    v: DVec2,
    dt: f64,
    m: f64,
    accel_fn: &dyn Fn(DVec2, DVec2, f64) -> DVec2,
    tolerance: f64,
) -> (DVec2, DVec2, f64) {
    // Butcher Tableau for Dormand-Prince
    // Coefficients for internal stages
    let a21 = 1.0/5.0;
    let a31 = 3.0/40.0;       let a32 = 9.0/40.0;
    let a41 = 44.0/45.0;      let a42 = -56.0/15.0;      let a43 = 32.0/9.0;
    let a51 = 19372.0/6561.0; let a52 = -25360.0/2187.0; let a53 = 64448.0/6561.0; let a54 = -212.0/729.0;
    let a61 = 9017.0/3168.0;  let a62 = -355.0/33.0;     let a63 = 46732.0/5247.0; let a64 = 49.0/176.0;   let a65 = -5103.0/18656.0;
    let a71 = 35.0/384.0;     let a73 = 500.0/1113.0;    let a74 = 125.0/192.0;    let a75 = -2187.0/6784.0; let a76 = 11.0/84.0;

    // 4th order (for error estimation)
    let e1 = 35.0/384.0 - 5179.0/57600.0;
    let e3 = 500.0/1113.0 - 7571.0/16695.0;
    let e4 = 125.0/192.0 - 393.0/640.0;
    let e5 = -2187.0/6784.0 - (-92097.0/339200.0);
    let e6 = 11.0/84.0 - 187.0/2100.0;
    let e7 = -1.0/40.0;

    // k1: Initial slope
    let k1_v = v;
    let k1_a = accel_fn(x, v, 0.0);

    // k2
    let k2_v = v + k1_a * (dt * a21);
    let k2_a = accel_fn(x + k1_v * (dt * a21), k2_v, dt * (1.0/5.0));

    // k3
    let k3_v = v + k1_a * (dt * a31) + k2_a * (dt * a32);
    let k3_a = accel_fn(x + k1_v * (dt * a31) + k2_v * (dt * a32), k3_v, dt * (3.0/10.0));

    // k4
    let k4_v = v + k1_a * (dt * a41) + k2_a * (dt * a42) + k3_a * (dt * a43);
    let k4_a = accel_fn(x + k1_v * (dt * a41) + k2_v * (dt * a42) + k3_v * (dt * a43), k4_v, dt * (4.0/5.0));

    // k5
    let k5_v = v + k1_a * (dt * a51) + k2_a * (dt * a52) + k3_a * (dt * a53) + k4_a * (dt * a54);
    let k5_a = accel_fn(x + k1_v * (dt * a51) + k2_v * (dt * a52) + k3_v * (dt * a53) + k4_v * (dt * a54), k5_v, dt * (8.0/9.0));

    // k6
    let k6_v = v + k1_a * (dt * a61) + k2_a * (dt * a62) + k3_a * (dt * a63) + k4_a * (dt * a64) + k5_a * (dt * a65);
    let k6_a = accel_fn(x + k1_v * (dt * a61) + k2_v * (dt * a62) + k3_v * (dt * a63) + k4_v * (dt * a64) + k5_v * (dt * a65), k6_v, dt);

    // Final result (5th order estimate)
    let next_x = x + (k1_v * a71 + k3_v * a73 + k4_v * a74 + k5_v * a75 + k6_v * a76) * dt;
    let next_v = v + (k1_a * a71 + k3_a * a73 + k4_a * a74 + k5_a * a75 + k6_a * a76) * dt;

    // Error estimation (the difference between 5th and 4th order)
    // We only check position error for simplicity, but could check velocity too
    let err_vec = (k1_v * e1 + k3_v * e3 + k4_v * e4 + k5_v * e5 + k6_v * e6 + next_v * e7) * dt;
    let error = err_vec.length();

    // Adaptive step size logic
    let mut next_dt = dt;
    if error > 0.0 {
        // Safety factor 0.9 to prevent oscillating steps
        let optimal_dt = dt * 0.9 * (tolerance / error).powf(0.2);
        next_dt = optimal_dt.clamp(dt * 0.1, dt * 5.0);
    }

    if error <= tolerance || dt < 1e-6 {
        // Step accepted
        (next_x, next_v, next_dt)
    } else {
        // Step rejected, try again with smaller dt
        rkdp_step(x, v, next_dt, m, accel_fn, tolerance)
    }
}

pub fn saba4_step(
    t: f64,
    x: DVec2,
    v: DVec2,
    dt: f64,
    m: f64,
    force: &dyn Fn(f64, DVec2) -> DVec2,
) -> (DVec2, DVec2) {
    let a = |t: f64, x: DVec2| force(t, x) / m;

    // Yoshida coefficients
    let w1: f64 = 1.35120719195966;
    let w0: f64 = -1.70241438391932;

    let a1 = w1 / 2.0;
    let a2 = (w0 + w1) / 2.0;
    let a3 = a2;
    let a4 = a1;

    let b1 = w1;
    let b2 = w0;
    let b3 = w1;

    let mut x = x;
    let mut v = v;
    let mut t = t;

    // A1 (Drift)
    x += v * (a1 * dt);
    t += a1 * dt;

    // B1 (Kick)
    v += a(t, x) * (b1 * dt);

    // A2
    x += v * (a2 * dt);
    t += a2 * dt;

    // B2
    v += a(t, x) * (b2 * dt);

    // A3
    x += v * (a3 * dt);
    t += a3 * dt;

    // B3
    v += a(t, x) * (b3 * dt);

    // A4
    x += v * (a4 * dt);
    //t += a4 * dt;

    (x, v)
}