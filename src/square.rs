use crate::math::*;
#[repr(C)] #[derive(Clone)]
pub struct Vertex {
    pub pos: Vec2,
    uv: Vec2,
}

#[derive(Clone)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
pub fn rectangle(width: f32, height: f32, pos: Vec2) -> Polygon {
    let vertices: Vec<Vertex> = vec![
        Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, uv: Vec2 { x: 0., y: 0. } }, // Bottom Left
        Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 0. } }, // Bottom Right
        Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 1. } }, // Top Right
        Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 0., y: 1. } }, // Top Left
    ];

    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];
    Polygon {
        vertices,
        indices,
    }
}

pub fn triangle(width: f32, height: f32, pos: Vec2) -> Polygon {
    let vertices: Vec<Vertex> = vec![
        Vertex { pos : Vec2 { x: -width/2.0 + pos.x, y: -height/2.0 + pos.y }, uv: Vec2 { x: 0., y: 0. } }, // Bottom Left
        Vertex { pos : Vec2 { x:  width/2.0 + pos.x, y: -height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 0. } }, // Bottom Right
        Vertex { pos : Vec2 { x:  pos.x, y:  height/2.0 + pos.y  }, uv: Vec2 { x: 1., y: 1. } }, // Top
    ];

    let indices: Vec<u16> = vec![0, 1, 2];
    Polygon {
        vertices,
        indices,
    }
}





