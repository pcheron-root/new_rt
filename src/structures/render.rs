
// use minifb::{Key, Window};
use minifb::Window;

// use crate::{Camera, Canvas, Direction, World};
use crate::{Camera, Canvas, World};

pub struct Renderer {
    pub window: Window,
    pub canvas: Canvas,
    pub world: World,
    pub camera: Camera,
    pub size: (usize, usize),
}

impl Renderer {
    pub fn render(&mut self, canvas: &mut Canvas, world: &World, camera: &Camera) {
        
    }

}
