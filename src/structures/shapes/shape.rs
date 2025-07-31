
use crate::{Sphere, Ray, LocalIntersection, Intersect};

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
        }
    }
}
