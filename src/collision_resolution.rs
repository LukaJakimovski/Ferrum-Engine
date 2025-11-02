use glam::Vec2;
use crate::collision_detection::{find_contact_points, sat_collision};
use crate::physics::PhysicsSystem;
use crate::Rigidbody;

impl PhysicsSystem {
    pub fn separate_into_section(&mut self) -> Vec<Vec<u16>> {
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
        rad_max *= 2.0;

        let mut x_sections: usize = ((x_max - x_min) / rad_max).ceil() as usize;
        if x_sections > 256 {
            x_sections = 256
        } else if x_sections == 0 {
            x_sections = 1
        }
        let mut y_sections: usize = ((y_max - y_min) / rad_max).ceil() as usize;
        if y_sections > 256 {
            y_sections = 256
        } else if y_sections == 0 {
            y_sections = 1
        }
        let section_count: usize = x_sections * y_sections;
        let x_range = x_max - x_min;
        let mut x_interval = x_range / x_sections as f32;
        if x_interval <= 0.0001 {
            x_interval = 0.0001
        }
        let y_range = y_max - y_min;
        let mut y_interval = y_range / y_sections as f32;
        if y_interval <= 0.0001 {
            y_interval = 0.0001
        }

        let mut sections: Vec<Vec<u16>> = Vec::with_capacity(x_sections);
        for _i in 0..section_count {
            sections.push(vec![]);
        }
        if section_count == 0 {
            return sections;
        }

        for i in 0..self.polygons.len() {
            let x_index: usize = (self.polygons[i].center.x / x_interval) as usize;
            let y_index: usize = (self.polygons[i].center.y / y_interval) as usize;
            let mut index = x_index + y_index * x_sections;
            if index >= sections.len() {
                index = sections.len() - 1
            }
            sections[index].push(i as u16);
        }

        let x = x_sections;
        for i in 0..sections.len() - 1 {
            let (left, right) = sections.split_at_mut(i + 1);
            let current = &mut left[i];

            let is_not_right_edge = x > 1 && (i + 1) % x != 0;
            let is_not_left_edge = x > 1 && i % x != 0;
            let row_valid = i + x < section_count;
            // Right neighbor
            if is_not_right_edge {
                current.extend_from_slice(&right[0]);
            }
            // Down neighbor
            if row_valid {
                current.extend_from_slice(&right[x - 1]);
            }
            // Down-right neighbor
            if row_valid && is_not_right_edge {
                current.extend_from_slice(&right[x]);
            }
            // Down-left neighbor
            if row_valid && is_not_left_edge {
                current.extend_from_slice(&right[x - 2]);
            }
        }
        sections
    }

    pub fn collision_resolution(&mut self) {
        let sections = self.separate_into_section();
        for section in sections {
            for i in 0..section.len() {
                if !self.polygons[section[i] as usize].collision {
                    continue;
                };
                for j in i + 1..section.len() {
                    if !self.polygons[section[j] as usize].collision {
                        continue;
                    } else if self.polygons[section[i] as usize].connected_anchors.contains(&(section[j] as usize)) {
                        continue;
                    } else if section[j] > section[i] {
                        let (left, right) = self.polygons.split_at_mut(section[j] as usize);
                        let a = &mut left[section[i] as usize];
                        let b = &mut right[0];
                        Self::check_and_resolve(a, b);
                    } else if section[i] > section[j] {
                        let (left, right) = self.polygons.split_at_mut(section[i] as usize);
                        let a = &mut left[section[j] as usize];
                        let b = &mut right[0];
                        Self::check_and_resolve(a, b);
                    }
                }
            }
        }
    }
    fn resolve_contact_velocity(body1: &mut Rigidbody, body2: &mut Rigidbody, contact: Vec2, normal: Vec2) {
        // Effective masses
        let m1 = if body1.is_static { 0.0 } else { 1.0 / body1.mass };
        let m2 = if body2.is_static { 0.0 } else { 1.0 / body2.mass };
        let i1 = if body1.is_static { 0.0 } else { 1.0 / body1.moment_of_inertia };
        let i2 = if body2.is_static { 0.0 } else { 1.0 / body2.moment_of_inertia };

        // Contact offsets
        let r1 = contact - body1.center;
        let r2 = contact - body2.center;

        // Velocities at contact
        let v1 = body1.velocity + r1.perp() * body1.angular_velocity;
        let v2 = body2.velocity + r2.perp() * body2.angular_velocity;
        let rv = v2 - v1;

        // Normal impulse
        let vel_n = rv.dot(normal);

        // Restitution OFF for resting contacts (prevents bounciness in stacks)
        let restitution_vel_threshold = 1e-2; // tweak
        let mut e = (body1.restitution + body2.restitution) * 0.5;
        if vel_n.abs() < restitution_vel_threshold { e = 0.0; }

        if vel_n > 0.0 {
            return; // separating
        }

        let rn1 = r1.perp_dot(normal);
        let rn2 = r2.perp_dot(normal);
        let k_normal = m1 + m2 + rn1*rn1*i1 + rn2*rn2*i2;

        let jn = -(1.0 + e) * vel_n / k_normal;
        let impulse_n = normal * jn;

        // Apply normal impulse
        body1.velocity -= impulse_n * m1;
        body2.velocity += impulse_n * m2;
        body1.angular_velocity -= rn1 * jn * i1;
        body2.angular_velocity += rn2 * jn * i2;

        // Friction (Coulomb) — uses same effective mass in tangent dir
        //let tangent = (rv - normal * vel_n).normalize_or_zero();
        //let vel_t = rv.dot(tangent);
        //let k_tangent = m1 + m2
        //    + (r1.perp_dot(tangent)).powi(2) * i1
        //    + (r2.perp_dot(tangent)).powi(2) * i2;

        //let jt = -vel_t / k_tangent;
        //let mu = (body1.friction + body2.friction) * 0.5;

        // Clamp friction to Coulomb cone
        //let jt_clamped = jt.clamp(-jn * mu, jn * mu);
        //let impulse_t = tangent * jt_clamped;

        //body1.velocity -= impulse_t * m1;
        //body2.velocity += impulse_t * m2;
        //body1.angular_velocity -= r1.perp_dot(impulse_t) * i1;
        //body2.angular_velocity += r2.perp_dot(impulse_t) * i2;
    }

    fn positional_correction_pair(
        body1: &mut Rigidbody,
        body2: &mut Rigidbody,
        normal: Vec2,
        penetration: f32,
        contact_count: usize,
    ) {
        if penetration <= 0.0 || contact_count == 0 { return; }

        // Treat static bodies as infinite mass
        let inv_m1 = if body1.is_static { 0.0 } else { 1.0 / body1.mass };
        let inv_m2 = if body2.is_static { 0.0 } else { 1.0 / body2.mass };
        let inv_m_sum = inv_m1 + inv_m2;
        if inv_m_sum == 0.0 { return; }


        // Share the penetration across contacts
        let correction = normal * penetration.clamp(0.0, 0.05) * 0.8; // 80% correction, up to 5cm
        body1.translate(-correction * (inv_m1 / inv_m_sum));
        body2.translate(correction * (inv_m2 / inv_m_sum));
    }


    pub fn check_and_resolve(body1: &mut Rigidbody, body2: &mut Rigidbody) {
        let result = sat_collision(&body1, &body2);
        // assume: result[1].y != 0 indicates collision, result[0] = axis, result[1].x = depth
        if result[1].y == 0.0 { return; }

        // Ensure axis points from body1 -> body2 (have SAT return this if possible)
        let mut normal = result[0].normalize();
        if normal.dot(body2.center - body1.center) < 0.0 {
            normal = -normal;
        }
        let penetration = result[1].x;

        // Build manifold
        let contacts = find_contact_points(&body1, &body2, &result);
        if contacts.is_empty() { return; }

        // --- Velocity solver (sequential impulses). Iterate for stacks.
        const VEL_ITERS: usize = 1; // tweak (4–10)
        for _ in 0..VEL_ITERS {
            for &c in &contacts {
                Self::resolve_contact_velocity(body1, body2, c, normal);
            }
        }

        // --- Single positional correction for the pair
        Self::positional_correction_pair(body1, body2, normal, penetration, contacts.len());
    }


}