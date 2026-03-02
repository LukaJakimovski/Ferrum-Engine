use glam::{Vec2, DVec2, DMat2};
use crate::color::ColorRGBA;
use crate::ode_solver::{rk4_angular_step, dormand_prince_step};
#[derive(Clone, Default, Debug)]
pub struct Rigidbody {
    pub center: DVec2,
    pub vertices: Vec<Vec2>,
    pub color: ColorRGBA,
    pub indices: Vec<u32>,
    pub radius: f32,
    pub mass: f64,
    pub velocity: DVec2,
    pub angular_velocity: f64,
    pub moment_of_inertia: f64,
    pub area: f64,
    pub restitution: f32,
    pub force: DVec2,
    pub gravity_force: Vec2,
    pub torque: f64,
    pub angle: f64,
    pub collision: bool,
    pub gravity_multiplier: f64,
    pub eternal: bool,
    pub connected_anchors: Vec<usize>,
    pub is_static: bool,
}
impl Rigidbody {
    pub fn rectangle(
        width: f32,
        height: f32,
        pos: DVec2,
        mass: f64,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let vertices: Vec<Vec2> = vec![
            Vec2 {
                x: -width / 2.0 + pos.x as f32,
                y: -height / 2.0 + pos.y as f32,
            }, // Bottom Left
            Vec2 {
                x: width / 2.0 + pos.x as f32,
                y: -height / 2.0 + pos.y as f32,
            }, // Bottom Right
            Vec2 {
                x: width / 2.0 + pos.x as f32,
                y: height / 2.0 + pos.y as f32,
            }, // Top Right
            Vec2 {
                x: -width / 2.0 + pos.x as f32,
                y: height / 2.0 + pos.y as f32,
            }, // Top Left
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass,
            velocity: DVec2::ZERO,
            radius: 0.0,
            center: DVec2::ZERO,
            vertices,
            color,
            indices,
            restitution,
            force: DVec2::ZERO,
            gravity_force: Vec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
            connected_anchors: vec![],
            is_static: false,
        };
        polygon.calculate_properties();
        polygon.center = pos;
        polygon
    }

    #[allow(dead_code)]
    pub fn triangle(
        width: f32,
        height: f32,
        pos: DVec2,
        mass: f64,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let vertices: Vec<Vec2> = vec![
            Vec2 {
                x: -width / 2.0 + pos.x as f32,
                y: -height / 2.0 + pos.y as f32,
            }, // Bottom Left
            Vec2 {
                x: width / 2.0 + pos.x as f32,
                y: -height / 2.0 + pos.y as f32,
            }, // Bottom Right
            Vec2 {
                x: pos.x as f32,
                y: height / 2.0 + pos.y as f32,
            }, // Top
        ];

        let indices: Vec<u32> = vec![0, 1, 2];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            color,
            moment_of_inertia: 0.0,
            mass,
            velocity: DVec2::ZERO,
            radius: 0.0,
            center: DVec2::ZERO,
            gravity_force: Vec2::ZERO,
            vertices,
            indices,
            restitution,
            force: DVec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
            connected_anchors: vec![],
            is_static: false,
        };
        polygon.calculate_properties();
        polygon
    }

    pub fn polygon(
        sides: u32,
        radius: f32,
        pos: DVec2,
        mass: f64,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let mut vertices: Vec<Vec2> = vec![];

        for i in 0..sides {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / sides as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let vertex = Vec2 {
                x: x + pos.x as f32,
                y: y + pos.y as f32,
            };
            vertices.push(vertex);
        }

        let mut indices: Vec<u32> = vec![];

        for i in 0..(sides - 1) {
            indices.push(i);
            indices.push(i + 1);
            indices.push(sides);
        }
        indices.push(0);
        indices.push(sides - 1);
        indices.push(sides);

        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass,
            velocity: DVec2::ZERO,
            color,
            radius,
            center: pos,
            gravity_force: Vec2::ZERO,
            vertices,
            indices,
            restitution,
            force: DVec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
            connected_anchors: vec![],
            is_static: false,
        };
        polygon.calculate_properties();
        polygon
    }

    pub fn rotation_matrix(&self) -> DMat2 {
        let (s, c) = self.angle.sin_cos();
        DMat2::from_cols(DVec2::new(c, s), DVec2::new(-s, c))
    }
    pub fn calculate_properties(&mut self) {
        self.calculate_area();
        self.calculate_center_of_mass();
        self.calculate_radius();
        self.calculate_moment_of_inertia();
    }
    pub fn calculate_radius(&mut self) {
        let mut max_radius = 0.0;
        for vertex in &self.vertices {
            let center = Vec2::new(self.center.x as f32, self.center.y as f32);
            let distance = vertex.distance(center);
            if distance > max_radius {
                max_radius = distance;
            }
        }
        self.radius = max_radius;
    }

    pub fn calculate_area(&mut self) {
        let n = self.vertices.len();
        let mut area = 0.0;

        for i in 0..n {
            let iv: &DVec2 = &DVec2::from(self.vertices[i]);
            let jv: &DVec2 = &DVec2::from(self.vertices[(i + 1) % n]);

            let cross = iv.perp_dot(*jv);
            area += cross;
        }
        area *= 0.5;
        self.area = area;
    }

    pub fn calculate_center_of_mass(&mut self) {
        let n = self.vertices.len();
        if n == 0 {
            self.center = DVec2::ZERO;
            return;
        }
        if n == 1 {
            self.center = DVec2::from(self.vertices[0]);
            return;
        }
        if n == 2 {
            // midpoint for a line segment
            self.center = DVec2::from((self.vertices[0] + self.vertices[1]) * 0.5);
            return;
        }

        let mut sum_cross = 0.0;
        let mut sum_cx = 0.0;
        let mut sum_cy = 0.0;

        for i in 0..n {
            let iv = DVec2::from(self.vertices[i]);
            let jv = DVec2::from(self.vertices[(i + 1) % n]);
            // explicit 2D cross product (scalar)
            let cross = iv.x * jv.y - iv.y * jv.x;
            sum_cross += cross;
            sum_cx += (iv.x + jv.x) * cross;
            sum_cy += (iv.y + jv.y) * cross;
        }

        // signed area = 0.5 * sum_cross
        let signed_area = 0.5 * sum_cross;

        // handle degenerate / nearly-zero area (colinear or numeric degenerate)
        if sum_cross.abs() < f64::EPSILON {
            // fallback: use average of vertices (or you could pick bounding midpoint)
            let mut avg = Vec2::ZERO;
            for &v in &self.vertices {
                avg += v;
            }
            self.center = DVec2::from(avg / (n as f32));
            // keep area as 0.0
            self.area = 0.0;
            return;
        }

        // centroid formula — note we use the signed sum_cross directly:
        // C = (1 / (6*A)) * sum((xi + xi+1) * cross)  where A = 0.5 * sum_cross
        // so 6*A = 3 * sum_cross -> centroid = sum_cx / (3 * sum_cross)
        let centroid_x = sum_cx / (3.0 * sum_cross);
        let centroid_y = sum_cy / (3.0 * sum_cross);

        self.center = DVec2::new(centroid_x, centroid_y);
        self.area = signed_area; // update stored area (signed); if you want positive area, store signed_area.abs()
    }


    pub fn calculate_moment_of_inertia(&mut self) {
        let n = self.vertices.len();

        let mut inertia = 0.0;

        for i in 0..n {
            let p0 = DVec2::from(self.vertices[i]);
            let p1 = DVec2::from(self.vertices[(i + 1) % n]);
            let cross = DVec2::perp_dot(p0, p1);

            let dx2 = p0.x * p0.x + p0.x * p1.x + p1.x * p1.x;
            let dy2 = p0.y * p0.y + p0.y * p1.y + p1.y * p1.y;
            inertia += cross * (dx2 + dy2);
        }
        let inertia_origin = inertia / 12.0;

        let cx = self.center.x;
        let cy = self.center.y;
        let inertia_centroid = inertia_origin - self.area * (cx * cx + cy * cy);

        self.moment_of_inertia = (inertia_centroid * (self.mass / self.area.abs())) as f64;
    }

    pub fn translate(&mut self, pos: DVec2) -> &mut Self {
        for vertex in &mut self.vertices {
            *vertex += Vec2::new(pos.x as f32, pos.y as f32);
        }
        self.center += pos;
        self
    }
    
    pub fn move_to(&mut self, pos: DVec2) -> &mut Self {
        let diff = pos - self.center;
        for vertex in &mut self.vertices {
            *vertex += Vec2::new(diff.x as f32, diff.y as f32);
        }
        self.center = pos;
        self
    }
    
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        for vertex in &mut self.vertices {
            let new_x = ((vertex.x - self.center.x as f32) * angle.cos()
                - (vertex.y - self.center.y as f32) * angle.sin())
                + self.center.x as f32;
            vertex.y = ((vertex.x - self.center.x as f32) * angle.sin()
                + (vertex.y - self.center.y as f32) * angle.cos())
                + self.center.y as f32;
            vertex.x = new_x;
        }
        self
    }

    pub fn change_color(&mut self, color: ColorRGBA) {
        self.color = color
    }

    pub fn update_rigidbody(&mut self, g: DVec2, dt: f64) {
        let force = |_: f64, _: DVec2, _: DVec2| g * self.mass * self.gravity_multiplier + self.force;
        let (new_x, new_v) = dormand_prince_step(0.0, self.center, self.velocity, dt, self.mass, &force);
        let force = |_: f64, _: f64, _: f64| 0.0;
        let (new_angle_b, new_omega_b) = rk4_angular_step(0.0, self.angle, self.angular_velocity, dt, self.moment_of_inertia, &force, );
        let diff = new_angle_b - self.angle;
        self.rotate(diff as f32);
        self.angle = new_angle_b;
        self.angular_velocity = new_omega_b;
        self.velocity = new_v;
        let diff = new_x - self.center;
        self.translate(diff);

    }
}