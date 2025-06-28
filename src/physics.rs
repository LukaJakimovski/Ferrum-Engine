use miniquad::date;
use crate::collision_detection::sat_collision;
use crate::color::Color;
use crate::math::Vec2;
use crate::render::World;
use crate::ode_solver::rk4_step;
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
        let start = date::now();
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
                    //let result = [Vec2 {x: 0.0, y: 0.0}, Vec2 {x: 0.0, y: 0.0}];
                    if result[1].y != 0.0 {
                        let direction = result[0].normalized();
                        let v1 = self.polygons[new_section[j]].velocity;
                        let v2 = self.polygons[new_section[k]].velocity;
                        let m1 = self.polygons[new_section[j]].mass;
                        let m2 = self.polygons[new_section[k]].mass;
                        let relative_velocity = v1 - v2;
                        let vel_along_normal = relative_velocity.dot(&direction);
                        let restitution = 1.0;
                        let impulse_magnitude = -(1.0 + restitution) * vel_along_normal / (1.0 / m1 + 1.0 / m2);
                        let impulse = direction * impulse_magnitude;
                        self.polygons[new_section[j]].velocity = v1 + impulse / m1;
                        self.polygons[new_section[k]].velocity = v2 - impulse / m2;

                        self.colliding_polygons.push(self.polygons[new_section[j]].clone());
                        let length = self.colliding_polygons.len();
                        self.colliding_polygons[length - 1].change_color(Color::red());
                        self.colliding_polygons.push(self.polygons[new_section[k]].clone());
                        let length = self.colliding_polygons.len();
                        self.colliding_polygons[length - 1].change_color(Color::red());
                    }
                }
            }
        }
        //println!("Collision resolution time: {:?}ms", (date::now() - start) * 1000.0);
    }
    pub fn update_physics(&mut self) {
        let mut kinetic_energy = 0.0;
        let g = Vec2 { x: 0.0, y: 9.81 };
        self.polygons[0].gravity_object = false;
        self.polygons[0].mass = 100000000000.0;
        for polygon in &mut self.polygons {
            let force = |_: f32, _: Vec2, _: Vec2| g;
            let (mut new_x, mut new_v) = rk4_step(0.0, polygon.center, polygon.velocity, 0.00001, polygon.mass, &force);
            for _i in 0..10{
                (new_x, new_v) = rk4_step(0.0, new_x, new_v, 0.00001, polygon.mass, &force);
            }
            polygon.velocity = new_v;
            let diff = polygon.center - new_x;
            polygon.translate(diff);
            kinetic_energy += 0.5 * polygon.mass * polygon.velocity.dot(&polygon.velocity);
        }
        self.collision_resolution();
        println!("Kinetic energy: {}", kinetic_energy);
    }
}