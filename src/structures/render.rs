
// use minifb::{Key, Window};
use minifb::{Key, Window, WindowOptions};

// use crate::{Camera, Canvas, Direction, World};
use crate::{Camera, Canvas, World, Point, Vector, Color, Matrix, Ray, Direction};

pub struct Renderer {
    pub window: Window,
    pub canvas: Canvas,
    pub world: World,
    pub camera: Camera,
    pub sky: Color,
    // pub size: (usize, usize),
    // enlever size pose pb avec la fonction render du projet d'origine

}

impl Renderer {
    pub fn new(canvas: Canvas, world: World) -> Result<Self, minifb::Error> { // return an error in case of window error
        match Window::new(
            "RT",
            canvas.width,
            canvas.height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            }
        ) {
            Ok(window) => {
                let camera = Camera::new(
                    Point::new(0., 0., 10.),
                    Vector::new(0., 0., -1.),
                    canvas.width as f32 / canvas.height as f32,
                    45f32.to_radians(),
                    0.1,
                    100.,
                );
                
                Ok(Self {
                    window,
                    canvas,
                    world,
                    camera,
                    sky: Color::new(0., 0., 0.), // a basculer dans le monde ?
                })
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_pixel(&mut self, ray: &Ray) -> Color {
        

        Color::new(1., 0., 0.)
    }

    pub fn update_image(&mut self) {

        let view = Matrix::view(
            self.camera.position,
            self.camera.position + self.camera.direction(),
            Vector::new(0., 1., 0.),
        );

        let projection = Matrix::projection(self.camera.fov, self.camera.aspect, self.camera.near, self.camera.far);

        let view_proj = projection * view;
        let inv_view_proj = view_proj.inverse().unwrap();

        for y in 0..self.canvas.height {
            let ndc_y = 1.0 - 2.0 * ((self.canvas.height - y) as f32 + 0.5) / self.canvas.height as f32;

            for x in 0..self.canvas.width {
                let ndc_x = 2.0 * (x as f32 + 0.5) / self.canvas.width as f32 - 1.0;

                let origin = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, -1.0);
                let target = inv_view_proj.clone() * Point::new(ndc_x, ndc_y, 1.0);

                let direction = (target - origin).normalize();

                let ray = Ray::new(Point::new(origin.x, origin.y, origin.z), direction);
                let color = self.get_pixel(&ray);
                self.canvas.write(x as usize, y as usize, color);
                // a completer ici
            }
        }
    }

    pub fn render(&mut self) -> Result<u32, minifb::Error> {
        while self.window.is_open() {
            if self.window.is_key_down(Key::Escape) {
                break;
            }
            let current_size = self.window.get_size();
            let size = (self.canvas.width, self.canvas.height);

            if size != current_size {
                self.canvas.resize(current_size.0, current_size.1);
                self.camera.resize(size.0 as f32 / size.1 as f32);
            }

            if self.window.is_key_down(Key::A) {
                self.camera.translate(Direction::Left);
            }
            if self.window.is_key_down(Key::D) {
                self.camera.translate(Direction::Right);
            }
            if self.window.is_key_down(Key::W) {
                self.camera.translate(Direction::Forward);
            }
            if self.window.is_key_down(Key::S) {
                self.camera.translate(Direction::Backward);
            }

            if self.window.is_key_down(Key::Up) {
                self.camera.rotate_x(1.);
            }
            if self.window.is_key_down(Key::Down) {
                self.camera.rotate_x(-1.);
            }

            if self.window.is_key_down(Key::Right) {
                self.camera.rotate_y(1.);
            }
            if self.window.is_key_down(Key::Left) {
                self.camera.rotate_y(-1.);
            }

            self.camera.update();

            self.update_image();

            let buffer = self.canvas.pixels();
            match self.window
                .update_with_buffer(&buffer, current_size.0, current_size.1) {
                    Ok(_) => {

                    }
                    Err(e) => {
                        eprintln!("Error uptating window: {:?}", e);
                        return Err(e);
                    }
                };
        }
        Ok(0)

    }

}
