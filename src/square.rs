use crate::color::Color;
use crate::math::*;

#[repr(C)] #[derive(Clone)] #[derive(Default)]
pub struct Vertex {
    pub pos: Vec2,
    uv: Vec2,
    pub color: Color,
}
#[derive(Clone)]
pub struct Polygon {
    pub center: Vec2,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
impl Polygon {
    pub fn calculate_center_of_mass(&self) -> Vec2 {
        let n = self.vertices.len();
        if n == 0{
            return Vec2::zero();
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
            return self.vertices[0].pos.clone();
        }
        let centroid_x = sum_cx / (6.0 * area);
        let centroid_y = sum_cy / (6.0 * area);
        Vec2::new(centroid_x, centroid_y)
    }

    pub fn rectangle(width: f32, height: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let vertices: Vec<Vertex> = vec![
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, uv: Vec2 { x: 0., y: 0. }, color }, // Bottom Left
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 0. }, color }, // Bottom Right
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 1. }, color }, // Top Right
            Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 0., y: 1. }, color }, // Top Left
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
        let polygon = Polygon {
            center: Vec2::zero(),
            vertices,
            indices,
        };
        polygon.calculate_center_of_mass();
        polygon
    }

    pub fn triangle(width: f32, height: f32, pos: Vec2) -> Self {
        let color = Color::random();
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, uv: Vec2 { x: 0., y: 0. }, color }, // Bottom Left
            Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 0. }, color }, // Bottom Right
            Vertex { pos : Vec2 { x:  pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 1. }, color }, // Top
        ];


        let indices: Vec<u16> = vec![0, 1, 2];
        let polygon = Polygon {
            center: Vec2::zero(),
            vertices,
            indices,
        };
        polygon.calculate_center_of_mass();
        polygon
    }
}





