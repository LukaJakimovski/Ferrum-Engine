use crate::World;

impl World{
    pub fn remove_rigidbody(&mut self, index: usize){
        self.polygons.remove(index);
        for i in 0..self.springs.len(){
            if self.springs[i].body_a == index || self.springs[i].body_b == index{
                self.springs.remove(i);
            }
            else if self.springs[i].body_a > index{
                self.springs[i].body_a -= 1;
            }
            else if self.springs[i].body_b > index{
                self.springs[i].body_b -= 1;
            }
        }
    }
}