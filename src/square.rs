use crate::color::Color;
use crate::math::*;

#[repr(C)] #[derive(Clone)] #[derive(Default)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
}
#[derive(Clone)] #[derive(Default)]
pub struct Polygon {
    pub center: Vec2,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub radius: f32,
}
impl Polygon {
    pub fn calculate_center_of_mass(&mut self){
        let n = self.vertices.len();
        if n == 0{
            self.center = Vec2::zero();
            return;
        }

        let mut area = 0.0;
        let mut sum_cx = 0.0;
        let mut sum_cy = 0.0;

        for i in 0..n{
            let iv: &Vec2 = &self.vertices[i].pos;
            let jv: &Vec2 = &self.vertices[(i + 1) % n].pos;

            let cross = iv.cross(&jv);
            area += cross;
            sum_cx += (iv.x + jv.x) * cross;
            sum_cy += (iv.y + jv.y) * cross;
        }

        area *= 0.5;
        if area == 0.0 {
            self.center = self.vertices[0].pos.clone();
            return;
        }
        let centroid_x = sum_cx / (6.0 * area);
        let centroid_y = sum_cy / (6.0 * area);
        self.center = Vec2::new(centroid_x, centroid_y);
    }

    pub fn rectangle(width: f32, height: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let vertices: Vec<Vertex> = vec![
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, color }, // Bottom Left
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, color }, // Bottom Right
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y:  height/2.0 + pos.y  }, color }, // Top Right
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y:  height/2.0 + pos.y  }, color }, // Top Left
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
        let mut polygon = Polygon {
            radius: 0.0,
            center: Vec2::zero(),
            vertices,
            indices,
        };
        polygon.calculate_radius();
        polygon.calculate_center_of_mass();
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
        let mut polygon = Polygon {
            radius: 0.0,
            center: Vec2::zero(),
            vertices,
            indices,
        };
        polygon.calculate_radius();
        polygon.calculate_center_of_mass();
        polygon
    }

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

        Polygon{
            radius,
            center: pos,
            vertices,
            indices,
        }
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
}





