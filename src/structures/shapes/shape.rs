
use crate::{Sphere, Ray, LocalIntersection, Intersect};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
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
