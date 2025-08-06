
use serde::{Deserialize, Serialize};

use crate::{Object, Light, Ray, Intersection};

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }
    
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray, n1: f32) -> Option<Intersection> {
        let mut closest_intersection = None;

        for object in &self.objects {
            let intersection = object.intersect(&ray, n1);

            if intersection.is_some() {
                if closest_intersection.is_none() {
                    closest_intersection = intersection
                } else {
                    if closest_intersection.clone().unwrap().t > intersection.clone().unwrap().t {
                        closest_intersection = intersection;
                    }
                }
            }
        }

        closest_intersection
    }

}
