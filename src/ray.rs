use crate::vec::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.get_normalized()
        }
    }

    pub fn from_to(from: Vec3, to: Vec3) -> Ray {
        Ray {
            origin: from,
            direction: (to - from).get_normalized()
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }
}
