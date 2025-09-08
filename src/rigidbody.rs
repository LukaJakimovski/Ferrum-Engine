use glam::Vec2;
use crate::color::ColorRGBA;
use crate::ode_solver::{rk4_angular_step, rk4_step};
#[derive(Clone, Default, Debug)]
pub struct Rigidbody {
    pub center: Vec2,
    pub vertices: Vec<Vec2>,
    pub color: ColorRGBA,
    pub indices: Vec<u32>,
    pub radius: f32,
    pub mass: f32,
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub moment_of_inertia: f32,
    pub area: f32,
    pub restitution: f32,
    pub force: Vec2,
    pub torque: f32,
    pub angle: f32,
    pub collision: bool,
    pub gravity_multiplier: f32,
    pub eternal: bool,
}
impl Rigidbody {
    pub fn rectangle(
        width: f32,
        height: f32,
        pos: Vec2,
        mass: f32,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let vertices: Vec<Vec2> = vec![
            Vec2 {
                x: -width / 2.0 + pos.x,
                y: -height / 2.0 + pos.y,
            }, // Bottom Left
            Vec2 {
                x: width / 2.0 + pos.x,
                y: -height / 2.0 + pos.y,
            }, // Bottom Right
            Vec2 {
                x: width / 2.0 + pos.x,
                y: height / 2.0 + pos.y,
            }, // Top Right
            Vec2 {
                x: -width / 2.0 + pos.x,
                y: height / 2.0 + pos.y,
            }, // Top Left
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass,
            velocity: Vec2::ZERO,
            radius: 0.0,
            center: Vec2::ZERO,
            vertices,
            color,
            indices,
            restitution,
            force: Vec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
        };
        polygon.calculate_properties();
        polygon
    }

    #[allow(dead_code)]
    pub fn triangle(
        width: f32,
        height: f32,
        pos: Vec2,
        mass: f32,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let vertices: Vec<Vec2> = vec![
            Vec2 {
                x: -width / 2.0 + pos.x,
                y: -height / 2.0 + pos.y,
            }, // Bottom Left
            Vec2 {
                x: width / 2.0 + pos.x,
                y: -height / 2.0 + pos.y,
            }, // Bottom Right
            Vec2 {
                x: pos.x,
                y: height / 2.0 + pos.y,
            }, // Top
        ];

        let indices: Vec<u32> = vec![0, 1, 2];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            color,
            moment_of_inertia: 0.0,
            mass,
            velocity: Vec2::ZERO,
            radius: 0.0,
            center: Vec2::ZERO,
            vertices,
            indices,
            restitution,
            force: Vec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
        };
        polygon.calculate_properties();
        polygon
    }

    pub fn polygon(
        sides: u32,
        radius: f32,
        pos: Vec2,
        mass: f32,
        restitution: f32,
        color: ColorRGBA,
    ) -> Self {
        let mut vertices: Vec<Vec2> = vec![];

        for i in 0..sides {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / sides as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let vertex = Vec2 {
                x: x + pos.x,
                y: y + pos.y,
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
            velocity: Vec2::ZERO,
            color,
            radius,
            center: pos,
            vertices,
            indices,
            restitution,
            force: Vec2::ZERO,
            torque: 0.0,
            angle: 0.0,
            collision: true,
            gravity_multiplier: 1.0,
            eternal: false,
        };
        polygon.calculate_properties();
        polygon
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
            let distance = vertex.distance(self.center);
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
            let iv: &Vec2 = &self.vertices[i];
            let jv: &Vec2 = &self.vertices[(i + 1) % n];

            let cross = iv.perp_dot(*jv);
            area += cross;
        }
        area *= 0.5;
        self.area = area;
    }

    pub fn calculate_center_of_mass(&mut self) {
        let n = self.vertices.len();
        if n == 0 {
            self.center = Vec2::ZERO;
            return;
        }

        let mut sum_cx = 0.0;
        let mut sum_cy = 0.0;

        for i in 0..n {
            let iv: &Vec2 = &self.vertices[i];
            let jv: &Vec2 = &self.vertices[(i + 1) % n];

            let cross = iv.perp_dot(*jv);

            sum_cx += (iv.x + jv.x) * cross;
            sum_cy += (iv.y + jv.y) * cross;
        }

        if self.area == 0.0 {
            self.center = self.vertices[0].clone();
            return;
        }
        let centroid_x = sum_cx / (6.0 * self.area);
        let centroid_y = sum_cy / (6.0 * self.area);
        self.center = Vec2::new(centroid_x, centroid_y);
    }

    pub fn calculate_moment_of_inertia(&mut self) {
        let n = self.vertices.len();

        let mut inertia = 0.0;

        for i in 0..n {
            let p0 = self.vertices[i];
            let p1 = self.vertices[(i + 1) % n];
            let cross = Vec2::perp_dot(p0, p1);

            let dx2 = p0.x * p0.x + p0.x * p1.x + p1.x * p1.x;
            let dy2 = p0.y * p0.y + p0.y * p1.y + p1.y * p1.y;
            inertia += cross * (dx2 + dy2);
        }
        let inertia_origin = inertia / 12.0;

        let cx = self.center.x;
        let cy = self.center.y;
        let inertia_centroid = inertia_origin - self.area * (cx * cx + cy * cy);

        self.moment_of_inertia = inertia_centroid * (self.mass / self.area.abs());
    }

    pub fn translate(&mut self, pos: Vec2) -> &mut Self {
        for vertex in &mut self.vertices {
            *vertex += pos;
        }
        self.center += pos;
        self
    }
    
    pub fn move_to(&mut self, pos: Vec2) -> &mut Self {
        let diff = pos - self.center;
        for vertex in &mut self.vertices {
            *vertex += diff;
        }
        self.center += diff;
        self
    }
    
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        for vertex in &mut self.vertices {
            let new_x = ((vertex.x - self.center.x) * angle.cos()
                - (vertex.y - self.center.y) * angle.sin())
                + self.center.x;
            vertex.y = ((vertex.x - self.center.x) * angle.sin()
                + (vertex.y - self.center.y) * angle.cos())
                + self.center.y;
            vertex.x = new_x;
        }
        self
    }

    pub fn change_color(&mut self, color: ColorRGBA) {
        self.color = color
    }

    pub fn update_rigidbody(&mut self, g: Vec2, dt: f32) {
        let force = |_: f32, _: Vec2, _: Vec2| g * self.mass * self.gravity_multiplier + self.force;
        let (new_x, new_v) = rk4_step(0.0, self.center, self.velocity, dt, self.mass, &force);
        let force = |_: f32, _: f32, _: f32| 0.0;
        let (new_angle_b, new_omega_b) = rk4_angular_step(
            0.0,
            self.angle,
            self.angular_velocity,
            dt,
            self.moment_of_inertia,
            &force,
        );
        let diff = new_angle_b - self.angle;
        self.rotate(diff);
        self.angle = new_angle_b;

        self.angular_velocity = new_omega_b;
        self.velocity = new_v;
        let diff = new_x - self.center;
        self.translate(diff);
    }

    pub fn calculate_energy(&self) -> f64 {
        let mut kinetic_energy = 0.0;
        kinetic_energy += 0.5 * self.mass * self.velocity.dot(self.velocity);
        kinetic_energy +=
            0.5 * self.moment_of_inertia * self.angular_velocity * self.angular_velocity;
        if kinetic_energy < 0.0 {
            return 0.0;
        }
        kinetic_energy as f64
    }
}