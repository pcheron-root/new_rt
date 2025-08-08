
use serde::{Deserialize, Serialize};

use crate::{Object, Light, Ray, Intersection, Point, Vector, Color};

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

    // shadow and light
    pub fn lighting(
        obj: &Object,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        shadowed: bool,
    ) -> Color {
        let effective_color;
        // if obj.material.pattern.is_some() {
        //     effective_color = obj
        //         .material
        //         .pattern
        //         .clone()
        //         .unwrap()
        //         .stripe_at_object(obj, point);
        // } else {
            effective_color = obj.material.color * light.intensity;
        // }

        let lightv = (light.position - *point).normalize();

        let ambient = effective_color * obj.material.ambient;
        let light_dot_normal = lightv.dot(normalv);

        if light_dot_normal < 0. || shadowed == true {
            return ambient;
        }

        let diffuse = effective_color * obj.material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0. {
            return ambient + diffuse;
        } else {
            let factor = reflect_dot_eye.powf(obj.material.shininess);
            let specular = light.intensity * obj.material.specular * factor;
            return ambient + diffuse + specular;
        }
    }

}
