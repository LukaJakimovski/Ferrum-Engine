use crate::collision_detection::{find_contact_points, sat_collision};
use crate::math::Vec2;
use crate::world::World;
use crate::ode_solver::rk4_step;
use crate::Rigidbody;

impl World {
    pub fn collision_resolution(&mut self) {
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;
        for i in 0..self.polygons.len() {
            if self.polygons[i].center.x < x_min {
                x_min = self.polygons[i].center.x;
            }
            if self.polygons[i].center.x > x_max {
                x_max = self.polygons[i].center.x;
            }
            if self.polygons[i].center.y < y_min {
                y_min = self.polygons[i].center.y;
            }
            if self.polygons[i].center.y > y_max {
                y_max = self.polygons[i].center.y;
            }
        }
        const X_SECTIONS: usize = 1;
        const Y_SECTIONS: usize = 1;
        const SECTION_COUNT: usize = X_SECTIONS * Y_SECTIONS;
        let x_range = x_max - x_min + 0.05;
        let x_interval = x_range / X_SECTIONS as f32;
        let y_range = y_max - y_min + 0.05;
        let y_interval = y_range / Y_SECTIONS as f32;
        let mut sections: [Vec<usize>; X_SECTIONS * Y_SECTIONS] = [const { vec![] }; X_SECTIONS * Y_SECTIONS];
        for i in 0..self.polygons.len() {
            let x_index: usize = (self.polygons[i].center.x / x_interval) as usize;
            let y_index: usize = (self.polygons[i].center.y / y_interval) as usize;
            let mut index = x_index + y_index * X_SECTIONS;
            if index >= sections.len() { index = sections.len() - 1}
            sections[index].push(i);
        }
        for i in 0..sections.len() {
            let mut new_section: Vec<usize> = vec![];
            new_section.extend(sections[i].clone());
            if (i as i32) < SECTION_COUNT as i32 - 1 { new_section.extend(sections[i + 1].clone()); }; // Right
            if (i as i32) < SECTION_COUNT as i32 - X_SECTIONS as i32 { new_section.extend(sections[i + X_SECTIONS].clone()); }; // Down
            if (i as i32) < SECTION_COUNT as i32 - X_SECTIONS as i32 - 1 { new_section.extend(sections[i + X_SECTIONS + 1].clone()); }; // Down right
            if (i as i32) < SECTION_COUNT as i32 - X_SECTIONS as i32 { new_section.extend(sections[i + X_SECTIONS - 1].clone()); }; // Down left
            for j in 0..new_section.len() {
                for k in j+1..new_section.len() {
                    let result = sat_collision(&self.polygons[new_section[j]], &self.polygons[new_section[k]]);
                    if result[1].y != 0.0 {

                        let polygon1 = &self.polygons[new_section[j]];
                        let polygon2 = &self.polygons[new_section[k]];
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
                        self.polygons[new_section[j]].translate(-normal * penetration * (m1 / m_total));
                        self.polygons[new_section[k]].translate(normal * penetration * (m2 / m_total));

                        // Impulse
                        let polygon1 = &self.polygons[new_section[j]];
                        let polygon2 = &self.polygons[new_section[k]];

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

                        self.polygons[new_section[j]].velocity = v1 - impulse_vector * m1;
                        self.polygons[new_section[k]].velocity = v2 + impulse_vector * m2;
                        if self.parameters.angular_velocity == true{
                            self.polygons[new_section[j]].angular_velocity = self.polygons[new_section[j]].angular_velocity - r1.cross(&impulse_vector) * i1;
                            self.polygons[new_section[k]].angular_velocity = self.polygons[new_section[k]].angular_velocity + r2.cross(&impulse_vector) * i2;
                        }
                        self.collisions += 1;
                    }
                }
            }
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
            spring.apply(self.delta_time as f32, self.delta_time as f32);
        }
        println!("{:?}", kinetic_energy);
        self.collision_resolution();
    }
}