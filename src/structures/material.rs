use crate::{Color, Pattern};
use std::default::Default;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, )]
pub struct Material {
    pub color: Color,
    pub shininess: f32,  // between 10 and 200
    pub ambient: f32,    // between 0 and 1
    pub diffuse: f32,    // between 0 and 1
    pub specular: f32,   // between 0 and 1
    pub reflective: f32, // between 0 and 1
    pub refractive_index: f32,
    pub pattern: Option<Pattern>,
    pub transparency: f32,
}

impl Default for Material {
    fn default() -> Self {
        let shininess = 200.0;
        let color = Color::new(1.0, 1.0, 1.0);
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let reflective = 0.0;
        let pattern = None;
        let refractive_index = 1.0;
        let transparency = 0.;

        Self {
            shininess,
            specular,
            color,
            ambient,
            diffuse,
            pattern,
            reflective,
            refractive_index,
            transparency,
        }
    }
}

impl Material {
    pub fn new() -> Self {
        Material::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    pub fn reflective(mut self, reflective: f32) -> Self {
        self.reflective = reflective.clamp(0.0, 1.0);

        self
    }

    pub fn specular(mut self, specular: f32) -> Self {
        self.specular = specular.clamp(0.0, 1.0);

        self
    }

    pub fn diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse.clamp(0.0, 1.0);

        self
    }

    pub fn shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess.clamp(10.0, 200.0);

        self
    }

    pub fn ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient.clamp(0.0, 1.0);

        self
    }

    //
    // pub fn pattern(mut self, pattern: Pattern) -> Self {
    //     self.pattern = Some(pattern);

    //     self
    // }
}
