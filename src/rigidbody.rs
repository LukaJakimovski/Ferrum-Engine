use crate::color::Color;
use crate::math::*;
use crate::ode_solver::rk4_step;

#[repr(C)] #[derive(Clone)] #[derive(Default)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
}
#[derive(Clone)] #[derive(Default)]
pub struct Rigidbody {
    pub center: Vec2,
    pub vertices: Vec<Vertex>,
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
}
impl Rigidbody {
    pub fn rectangle(width: f32, height: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let vertices: Vec<Vertex> = vec![
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, color }, // Bottom Left
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, color }, // Bottom Right
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y:  height/2.0 + pos.y  }, color }, // Top Right
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y:  height/2.0 + pos.y  }, color }, // Top Left
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass: 1.0,
            velocity: Vec2::zero(),
            radius: 0.0,
            center: Vec2::zero(),
            vertices,
            indices,
            restitution: 1.0,
            force: Vec2::zero(),
            torque: 0.0,
            angle: 0.0,
        };
        polygon.calculate_area();
        polygon.calculate_radius();
        polygon.calculate_center_of_mass();
        polygon.calculate_moment_of_inertia();
        polygon
    }

    #[allow(dead_code)]
    pub fn triangle(width: f32, height: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, color }, // Bottom Left
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, color }, // Bottom Right
            Vertex { pos : Vec2 { x:  pos.x, y:  height/2.0 + pos.y  }, color }, // Top
        ];


        let indices: Vec<u32> = vec![0, 1, 2];
        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass: 1.0,
            velocity: Vec2::zero(),
            radius: 0.0,
            center: Vec2::zero(),
            vertices,
            indices,
            restitution: 1.0,
            force: Vec2::zero(),
            torque: 0.0,
            angle: 0.0,
        };
        polygon.calculate_area();
        polygon.calculate_radius();
        polygon.calculate_center_of_mass();
        polygon
    }
    #[allow(dead_code)]
    pub fn polygon(sides: u32, radius: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let mut vertices: Vec<Vertex> = vec![];

        for i in 0..sides {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / sides as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let vertex = Vertex { pos: Vec2 { x: x + pos.x, y: y + pos.y }, color };
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
        indices.push(sides );

        let mut polygon = Rigidbody {
            angular_velocity: 0.0,
            area: 0.0,
            moment_of_inertia: 0.0,
            mass: 1.0,
            velocity: Vec2::zero(),
            radius,
            center: pos,
            vertices,
            indices,
            restitution: 1.0,
            force: Vec2::zero(),
            torque: 0.0,
            angle: 0.0,
        };
        polygon.calculate_area();
        polygon.calculate_moment_of_inertia();
        polygon
    }

    pub fn calculate_radius(&mut self){
        let mut max_radius = 0.0;
        for vertex in &self.vertices {
            let distance = vertex.pos.distance(&self.center);
            if distance > max_radius {
                max_radius = distance;
            }
        }
        self.radius = max_radius;
    }

    pub fn calculate_area(&mut self){
        let n = self.vertices.len();
        let mut area = 0.0;

        for i in 0..n{
            let iv: &Vec2 = &self.vertices[i].pos;
            let jv: &Vec2 = &self.vertices[(i + 1) % n].pos;

            let cross = iv.cross(&jv);
            area += cross;
        }
        area *= 0.5;
        self.area = area;
    }

    pub fn calculate_center_of_mass(&mut self){
        let n = self.vertices.len();
        if n == 0{
            self.center = Vec2::zero();
            return;
        }

        let mut sum_cx = 0.0;
        let mut sum_cy = 0.0;

        for i in 0..n{
            let iv: &Vec2 = &self.vertices[i].pos;
            let jv: &Vec2 = &self.vertices[(i + 1) % n].pos;

            let cross = iv.cross(&jv);

            sum_cx += (iv.x + jv.x) * cross;
            sum_cy += (iv.y + jv.y) * cross;
        }


        if self.area == 0.0 {
            self.center = self.vertices[0].pos.clone();
            return;
        }
        let centroid_x = sum_cx / (6.0 * self.area);
        let centroid_y = sum_cy / (6.0 * self.area);
        self.center = Vec2::new(centroid_x, centroid_y);
    }

    pub fn calculate_moment_of_inertia(&mut self){
        let n = self.vertices.len();

        let mut inertia = 0.0;

        for i in 0..n {
            let p0 = self.vertices[i].pos;
            let p1 = self.vertices[(i + 1) % n].pos;
            let cross = Vec2::cross(&p0, &p1);

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

    pub fn translate(&mut self, pos: Vec2) -> &mut Self{
        for vertex in &mut self.vertices {
            vertex.pos += pos;
        }
        self.center += pos;
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self{
        for vertex in &mut self.vertices {
            let new_x = ((vertex.pos.x - self.center.x) * angle.cos()  - (vertex.pos.y - self.center.y) * angle.sin()) + self.center.x;
            vertex.pos.y = ((vertex.pos.x - self.center.x) * angle.sin()  + (vertex.pos.y - self.center.y) * angle.cos()) + self.center.y;
            vertex.pos.x = new_x;
        }
        self
    }

    pub fn change_color(&mut self, color: Color){
        for vertex in &mut self.vertices {
            vertex.color = color;
        }
    }

    pub fn update_rigidbody(&mut self, g: Vec2, dt: f32) {
        let force = |_: f32, _: Vec2, _: Vec2| g + self.force;
        let (new_x, new_v) = rk4_step(0.0, self.center, self.velocity, dt, self.mass, &force);
        self.rotate(self.angular_velocity * dt);
        self.velocity = new_v;
        let diff = new_x - self.center;
        self.translate(diff);
    }
}





