use crate::rigidbody::Rigidbody;
use crate::math::Vec2;

fn get_axes(shape: &Rigidbody) -> Vec<Vec2>{
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

fn project(shape: &Rigidbody, axis: &Vec2) -> Vec2{
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

pub fn sat_collision(shape1: &Rigidbody, shape2: &Rigidbody) -> [Vec2; 2]{
    // Simple circle check
    if shape1.center.distance(&shape2.center) > shape1.radius + shape2.radius {
        return [Vec2 {x: -133.7, y: -133.7}, Vec2 {x: -133.7, y: 0.0}];
    }
    // Treat shapes with more than 15 vertices as circles
    if shape1.vertices.len() >= 16 && shape2.vertices.len() >= 16 {
        let delta = shape2.center - shape1.center;
        let dist = delta.magnitude();
        let overlap = shape1.radius + shape2.radius - dist;    // Total overlap amount

        return [delta.normalized(), Vec2 {x: overlap, y: 1.0}];
    }

    let mut overlap: f32 = 2.0_f32.powf(32.0);
    let mut smallest: Vec2 = Vec2 {x: 0.0, y: 0.0};
    let axes1: Vec<Vec2> = get_axes(shape1);
    let axes2: Vec<Vec2> = get_axes(shape2);
    let mut shape: f32 = 0.0;

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
                // Pointing away from shape1
                shape = 1.0;
            }
        }
    }

    for i in 0..axes2.len(){
        let axis = &axes2[i];

        let p1 = project(shape1, &axis);
        let p2 = project(shape2, &axis);

        if !overlaps(&p1, &p2){
            return [Vec2 {x: -133.7, y: -133.7}, Vec2 {x: -133.7, y: 0.0}];
        } else {
            let o: f32 = get_overlap(&p1, &p2);
            if o < overlap {
                overlap = o;
                smallest = axis.clone();
                // Pointing away from shape2
                shape = -1.0;
            }
        }
    }
    if overlap < 0.0001 {
        return [Vec2 {x: -133.7, y: -133.7}, Vec2 {x: -133.7, y: 0.0}];
    }
    [Vec2 {x: smallest.x, y: smallest.y}, Vec2 {x: overlap, y: shape}]
}
fn clip(v1: Vec2, v2: Vec2, normal: Vec2, offset: f32) -> Vec<Vec2> {
    let mut clipped = Vec::new();
    let d1 = normal.dot(&v1) - offset;
    let d2 = normal.dot(&v2) - offset;

    if d1 >= 0.0 {clipped.push(v1);};
    if d2 >= 0.0 {clipped.push(v2);};

    if d1 * d2 < 0.0 {
        let mut e = v2 - v1;

        let u = d1 / (d1 - d2);
        e = e * u;
        e = e + v1;

        clipped.push(e);
    }
    clipped
}
fn best_edge(polygon: &Rigidbody, normal: Vec2) -> (Vec2, Vec2, Vec2) {
    let c = polygon.vertices.len();
    let mut max = f32::MIN;
    let mut index = 0;
    for i in 0..c {
        let projection = normal.dot(&polygon.vertices[i].pos);
        if projection > max {
            max = projection;
            index = i
        }
    }

    let v = polygon.vertices[index].pos;
    let v1 = polygon.vertices[(index + 1) % c].pos;
    let v0 = polygon.vertices[(index + c - 1) % c].pos;

    let mut l = v - v1;
    let mut r = v - v0;

    l.normalize();
    r.normalize();

    if r.dot(&normal) <= l.dot(&normal) {
        (v0, v, v)
    } else {
        (v, v1, v)
    }
}



pub fn find_contact_points(polygon1: &Rigidbody, polygon2: &Rigidbody, mtv: &[Vec2; 2], ) -> Vec<Vec2> {
    let normal;
    if mtv[0].normalized().dot(&polygon1.center) < mtv[0].normalized().dot(&polygon2.center) {
        normal = mtv[0].normalized();
    }
    else {
        normal = -mtv[0].normalized();
    }
    let edge1 = best_edge(&polygon1, normal);
    let edge2 = best_edge(&polygon2, -normal);
    let edge1v = edge1.1 - edge1.0;
    let edge2v = edge2.1 - edge2.0;

    let ref_edge;
    let inc_edge;
    let mut flip = false;
    if edge1v.dot(&normal).abs() <= edge2v.dot(&normal).abs() {
        ref_edge = edge1;
        inc_edge = edge2;
    } else {
        ref_edge = edge2;
        inc_edge = edge1;
        flip = true;
    }

    let mut refv = ref_edge.1 - ref_edge.0;
    refv.normalize();

    let o1 = refv.dot(&ref_edge.0);
    let mut clipped = clip(inc_edge.0, inc_edge.1, refv, o1);
    if clipped.len() < 2 { return vec![Vec2::zero(), Vec2::zero()];};

    let mut ref_normal = Vec2::new(ref_edge.1.y - ref_edge.0.y, ref_edge.0.x - ref_edge.1.x).normalized();

    if flip {ref_normal = -ref_normal;};

    let max = ref_normal.dot(&ref_edge.2);

    if clipped.len() > 0 && ref_normal.dot(&clipped[0]) - max < 0.0 {
        clipped.remove(0);
    }
    if clipped.len() > 1 && ref_normal.dot(&clipped[1]) - max < 0.0 {
        clipped.remove(1);
    }

    clipped
}
