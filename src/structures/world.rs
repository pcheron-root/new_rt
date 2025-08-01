
use serde::{Deserialize, Serialize};

use crate::{Object, Light};

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
}
