
use crate::{Sphere, Ray, LocalIntersection, Intersect, Cube, Disk, Torus, Triangle};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Cube(Cube),
    Disk(Disk),
    Torus(Torus),
    Triangle(Triangle),
}

impl Shape {
    pub fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
            Shape::Cube(s) => s.intersect(ray),
            Shape::Disk(s) => s.intersect(ray),
            Shape::Torus(s) => s.intersect(ray),
            Shape::Triangle(s) => s.intersect(ray),
        }
    }
}
