use crate::collision_detection::{find_contact_points, sat_collision};
use crate::math::Vec2;
use crate::Rigidbody;
use crate::world::World;

impl World {
    pub fn collision_resolution(&mut self) {
        for i in 0..self.polygons.len() {
            for j in 0..self.springs.len(){
                let mut body1 = self.polygons[i].clone();
                let mut body2 = self.springs[j].body_a.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.polygons[i] = body1;
                self.springs[j].body_a = body2;

                let mut body1 = self.polygons[i].clone();
                let mut body2 = self.springs[j].body_b.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.polygons[i] = body1;
                self.springs[j].body_b = body2;

                let mut body1 = self.springs[j].body_a.clone();
                let mut body2 = self.springs[j].body_b.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.springs[j].body_a = body1;
                self.springs[j].body_b = body2;
            }

            for j in i+1..self.polygons.len() {
                let mut body1 = self.polygons[i].clone();
                let mut body2 = self.polygons[j].clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.polygons[i] = body1;
                self.polygons[j] = body2;
            }
        }

        for i in 0..self.springs.len() {
            for j in i+1..self.springs.len() {
                let mut body1 = self.springs[i].body_a.clone();
                let mut body2 = self.springs[j].body_a.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.springs[i].body_a = body1;
                self.springs[j].body_a = body2;

                let mut body1 = self.springs[i].body_a.clone();
                let mut body2 = self.springs[j].body_b.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.springs[i].body_a = body1;
                self.springs[j].body_b = body2;

                let mut body1 = self.springs[i].body_b.clone();
                let mut body2 = self.springs[j].body_a.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.springs[i].body_b = body1;
                self.springs[j].body_a = body2;

                let mut body1 = self.springs[i].body_b.clone();
                let mut body2 = self.springs[j].body_b.clone();
                self.check_and_resolve(&mut body1, &mut body2);
                self.springs[i].body_b = body1;
                self.springs[j].body_b = body2;
            }
        }
    }
    
    pub fn check_and_resolve(&mut self, body1: &mut Rigidbody, body2: &mut Rigidbody) {
        let result = sat_collision(&body1, &body2);
        if result[1].y != 0.0 {

            let polygon1 = &body1;
            let polygon2 = &body2;
            let v1 = polygon1.velocity;
            let v2 = polygon2.velocity;
            let m1 = 1.0 / polygon1.mass;
            let m2 = 1.0 / polygon2.mass;
            let m_total = m1 + m2;
            let penetration = result[1].x;
            let normal;
            if result[0].normalized().dot(&polygon1.center) < result[0].normalized().dot(&polygon2.center) {
                normal = result[0].normalized();
            }
            else {
                normal = -result[0].normalized();
            }
            // Contact Points
            let contact_points = find_contact_points(polygon1, polygon2, &result);
            let mut average_point = Vec2::zero();
            for point in &contact_points {
                average_point += *point;
            }
            average_point /= contact_points.len() as f32;
            //Move them out of each other
            body1.translate(-normal * penetration * (m1 / m_total));
            body2.translate(normal * penetration * (m2 / m_total));

            // Impulse
            let polygon1 = &body1;
            let polygon2 = &body2;

            let r1 = average_point - polygon1.center;
            let r2 = average_point - polygon2.center;

            let tangent_v1 = r1.perpendicular() * polygon1.angular_velocity;
            let tangent_v2 = r2.perpendicular() * polygon2.angular_velocity;
            let fv1 = v1 + tangent_v1;
            let fv2 = v2 + tangent_v2;
            let relative_velocity = fv2 - fv1;
            let vel_along_normal = relative_velocity.dot(&normal);

            let restitution: f32;
            if polygon1.restitution < polygon2.restitution {
                restitution = polygon1.restitution;
            } else{
                restitution = polygon2.restitution;
            }

            let i1 = 1.0 / polygon1.moment_of_inertia;
            let i2 = 1.0 / polygon2.moment_of_inertia;

            let rn1 = r1.cross(&normal);
            let rn2 = r2.cross(&normal);

            let angle_term1: f32;
            let angle_term2: f32;

            if self.parameters.angular_velocity == true{
                angle_term1 = (rn1 * rn1) * i1;
                angle_term2 = (rn2 * rn2) * i2;
            } else {
                angle_term1 = 0.0;
                angle_term2 = 0.0;
            }

            let impulse_magnitude = -(1.0 + restitution) * vel_along_normal / (m1 + m2 + angle_term1 + angle_term2);
            let impulse_vector = normal * impulse_magnitude;

            body1.velocity = v1 - impulse_vector * m1;
            body2.velocity = v2 + impulse_vector * m2;
            if self.parameters.angular_velocity == true{
                body1.angular_velocity = body1.angular_velocity - r1.cross(&impulse_vector) * i1;
                body2.angular_velocity = body2.angular_velocity + r2.cross(&impulse_vector) * i2;
            }
            self.collisions += 1;
        }
    }
    
    pub fn update_physics(&mut self) {
        let mut kinetic_energy = 0.0;
        let g: Vec2;
        if self.parameters.gravity == true{
            g = Vec2 { x: 0.0, y: -9.81 };
        } else{
            g = Vec2 { x: 0.0, y: 0.0 };
        }

        for polygon in &mut self.polygons {
            polygon.update_rigidbody(g, self.delta_time as f32);
        }
        
        for spring in &mut self.springs{
            spring.apply(self.delta_time as f32);
        }
        println!("{:?}", kinetic_energy);
        self.collision_resolution();
    }
}