use glam::Vec2;

pub fn rk4_step(
    t: f32,
    x: Vec2,
    v: Vec2,
    dt: f32,
    m: f32,
    force: &dyn Fn(f32, Vec2, Vec2) -> Vec2,
) -> (Vec2, Vec2) {
    let a = |t: f32, x: Vec2, v: Vec2| force(t, x, v) / m;

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
    t: f32,
    angle: f32,
    angular_velocity: f32,
    dt: f32,
    moment_of_inertia: f32,
    torque: &dyn Fn(f32, f32, f32) -> f32,
) -> (f32, f32) {
    let alpha = |t: f32, theta: f32, omega: f32| torque(t, theta, omega) / moment_of_inertia;

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
    t: f32,
    x: Vec2,
    v: Vec2,
    dt: f32,
    m: f32,
    force: &dyn Fn(f32, Vec2, Vec2) -> Vec2,
) -> ((Vec2, Vec2), (Vec2, Vec2)) {
    let a = |t: f32, x: Vec2, v: Vec2| force(t, x, v) / m;

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
    let x_star = x
        + dt * (k1_x * (5179.0 / 57600.0)
        + k3_x * (7571.0 / 16695.0)
        + k4_x * (393.0 / 640.0)
        + k5_x * (-92097.0 / 339200.0)
        + k6_x * (187.0 / 2100.0)
        + k7_x * (1.0 / 40.0));
    let v_star = v
        + dt * (k1_v * (5179.0 / 57600.0)
        + k3_v * (7571.0 / 16695.0)
        + k4_v * (393.0 / 640.0)
        + k5_v * (-92097.0 / 339200.0)
        + k6_v * (187.0 / 2100.0)
        + k7_v * (1.0 / 40.0));

    ((x_next, v_next), (x_next - x_star, v_next - v_star))
}