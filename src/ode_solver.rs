use crate::math::Vec2;

pub fn rk4_step(t: f32, x: Vec2, v: Vec2, dt: f32, m: f32, force: &dyn Fn(f32, Vec2, Vec2) -> Vec2) -> (Vec2, Vec2)
{
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