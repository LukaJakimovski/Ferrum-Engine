use crate::collision_detection::{find_contact_points, sat_collision};
use crate::math::Vec2;
use crate::{Parameters, Rigidbody};
use crate::world::World;

impl World {
    pub fn separate_into_section(&mut self) -> Vec<Vec<usize>>{
        let mut x_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_min = f32::MAX;
        let mut y_max = f32::MIN;
        let mut rad_max = f32::MIN;

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
            if self.polygons[i].radius > rad_max {
                rad_max = self.polygons[i].radius;
            }
        }
        if x_max > self.parameters.world_size{
            x_max = self.parameters.world_size;
        }
        if y_max > self.parameters.world_size{
            y_max = self.parameters.world_size;
        }
        if x_min < -self.parameters.world_size{
            x_min = -self.parameters.world_size;
        }
        if y_min < -self.parameters.world_size{
            y_min = -self.parameters.world_size;
        }

        let mut x_sections: usize = ((x_max - x_min) / rad_max).ceil() as usize;
        if x_sections > 256 {x_sections = 256}
        else if x_sections == 0 {x_sections = 1}
        let mut y_sections: usize = ((y_max - y_min) / rad_max).ceil() as usize;
        if y_sections > 256 {y_sections = 256}
        else if y_sections == 0 {y_sections = 1}
        let section_count: usize = x_sections * y_sections;
        let x_range = x_max - x_min;
        let mut x_interval = x_range / x_sections as f32;
        if x_interval <= 0.0001 {x_interval = 0.0001}
        let y_range = y_max - y_min;
        let mut y_interval = y_range / y_sections as f32;
        if y_interval <= 0.0001 {y_interval = 0.0001}

        let mut sections: Vec<Vec<usize>> = Vec::with_capacity(x_sections as usize);
        for _i in 0..section_count {
            sections.push(vec![]);
        }
        if section_count == 0{
            return sections;
        }

        for i in 0..self.polygons.len() {
            let x_index: usize = (self.polygons[i].center.x / x_interval) as usize;
            let y_index: usize = (self.polygons[i].center.y / y_interval) as usize;
            let mut index = x_index + y_index * x_sections as usize;
            if index >= sections.len() {index = sections.len() - 1}
            sections[index].push(i);
        }

        for i in 0..sections.len() - 1 {
            let (left, right) = sections.split_at_mut(i + 1);
            if (i as i32) < section_count as i32 && x_sections > 1{ left[i].extend_from_slice(&*right[0]); }; // Right
            if (i as i32) < section_count as i32 - x_sections as i32 { left[i].extend(&*right[x_sections - 1]); }; // Down
            if (i as i32) < section_count as i32 - x_sections as i32 - 1 && x_sections > 1{ left[i].extend(&*right[x_sections]); }; // Down right
            if (i as i32) < section_count as i32 - x_sections as i32 - 1 && x_sections > 1 { left[i].extend(&*right[x_sections - 2]); }; // Down left
        }
        sections
    }

    pub fn collision_resolution(&mut self) {

        let sections = self.separate_into_section();
        for section in sections {
            for i in 0..section.len() {
                if !self.polygons[section[i]].collision {continue;};
                for j in i+1..section.len() {
                    if !self.polygons[section[j]].collision {continue;};
                    if section[i] == 0 && section[j] == 0{
                        continue;
                    }
                    else if section[j] > section[i] {
                        let (left, right) = self.polygons.split_at_mut(section[j]);
                        let a = &mut left[section[i]];
                        let b = &mut right[0];
                        Self::check_and_resolve(&self.parameters, a, b);
                    }
                    else if section[i] > section[j] {
                        let (left, right) = self.polygons.split_at_mut(section[i]);
                        let a = &mut left[section[j]];
                        let b = &mut right[0];
                        Self::check_and_resolve(&self.parameters, a, b);
                    }

                }
            }
        }
    }
    
    pub fn check_and_resolve(parameters: &Parameters, body1: &mut Rigidbody, body2: &mut Rigidbody) {
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
            
            let restitution = (polygon1.restitution + polygon2.restitution) / 2.0;

            let i1 = 1.0 / polygon1.moment_of_inertia;
            let i2 = 1.0 / polygon2.moment_of_inertia;

            let rn1 = r1.cross(&normal);
            let rn2 = r2.cross(&normal);

            let angle_term1: f32;
            let angle_term2: f32;

            if parameters.angular_velocity == true{
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
            if parameters.angular_velocity == true{
                body1.angular_velocity = body1.angular_velocity - r1.cross(&impulse_vector) * i1;
                body2.angular_velocity = body2.angular_velocity + r2.cross(&impulse_vector) * i2;
            }
        }
    }
    
    pub fn update_physics(&mut self) {
        self.collision_resolution();
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
            spring.apply(self.delta_time as f32, &mut self.polygons);
        }
    }
}