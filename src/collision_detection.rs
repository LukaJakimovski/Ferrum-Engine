use crate::square::Polygon;
use crate::math::Vec2;

fn get_axes(shape: &Polygon) -> Vec<Vec2>{
    let mut axes: Vec<Vec2> = vec![];
    for i in 0..shape.vertices.len() {
        let p1: &Vec2 = &shape.vertices[i].pos;
        let p2: &Vec2 = &shape.vertices[if i + 1 == shape.vertices.len() { 0 } else { i + 1 }].pos;

        let edge = p1 - p2;
        let normal = edge.perpendicular();
        axes.push(normal);
    }
    axes
}

fn project(shape: &Polygon, axis: &Vec2) -> Vec2{
    let mut min: f32 = axis.dot(&shape.vertices[0].pos);
    let mut max: f32 = min;

    for i in 1..shape.vertices.len() {
        let p: f32 = axis.dot(&shape.vertices[i].pos);
        if p < min{
            min = p;
        } else if p > max {
            max = p;
        }
    }
    Vec2 {x: min, y: max}
}

fn overlaps(interval1: &Vec2, interval2: &Vec2) -> bool{
    !(interval1.x > interval2.y || interval2.x > interval1.y)
}

fn get_overlap(interval1: &Vec2, interval2: &Vec2) -> f32{
    if overlaps(interval1, interval2){
        let max = |x: f32, y: f32| if x > y { x } else { y };
        let min = |x: f32, y: f32| if x < y { x } else { y };
        return min(interval1.y, interval2.y) - max(interval1.x, interval2.x)
    }
    0.0
}

pub fn sat_collision(shape1: &Polygon, shape2: &Polygon) -> [Vec2; 2]{
    let mut overlap: f32 = 2.0_f32.powf(32.0);
    let mut smallest: Vec2 = Vec2 {x: 0.0, y: 0.0};
    let axes1: Vec<Vec2> = get_axes(shape1);
    let axes2: Vec<Vec2> = get_axes(shape2);

    for i in 0..axes1.len(){
        let axis = &axes1[i];

        let p1 = project(shape1, &axis);
        let p2 = project(shape2, &axis);

        if !overlaps(&p1, &p2){
            return [Vec2 {x: -133.7, y: -133.7}, Vec2 {x: -133.7, y: 0.0}];
        } else {
            let o: f32 = get_overlap(&p1, &p2);
            if o < overlap {
                overlap = o;
                smallest = axis.clone();
            }
        }
    }

    for i in 0..axes2.len(){
        let axis = &axes2[i];

        let p1 = project(shape1, &axis);
        let p2 = project(shape2, &axis);

        if !overlaps(&p1, &p2){
            return [Vec2 {x: -133.7, y: -133.7}, Vec2 {x: -133.7, y: -133.7}];
        } else {
            let o: f32 = get_overlap(&p1, &p2);
            if o < overlap {
                overlap = o;
                smallest = axis.clone();
            }
        }
    }
    [Vec2 {x: smallest.x, y: smallest.y}, Vec2 {x: overlap, y: 1.0}]
}