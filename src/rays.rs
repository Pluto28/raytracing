use std::ops::Mul;

use crate::vector::Vec3;


// The book has a point3 alias to a vec3. I don't see why use that given they are
// the same thing and in rust it would essentially just have to be anoh
#[derive(Debug, Copy, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn origin(&self) -> Vec3 { 
        self.origin
    }

    fn direction(&self) -> Vec3 {
        self.direction
    }

    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction.mul(t)
    }
}
