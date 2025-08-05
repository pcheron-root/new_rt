
use std::ops::{Add, Sub, Mul};
use crate::{Point, Vector, Direction};

fn lerp<T>(start: T, end: T, t: f32) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T>,
{
    let t = t.clamp(0., 1.);

    start * (1. - t) + end * t
}

pub struct Camera {
    pub target: Point,
    pub position: Point,
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,

    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn new(
        position: Point,
        direction: Vector,
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let direction = direction.normalize();
        let pitch = direction.y.asin().to_degrees();
        let yaw = direction.z.atan2(direction.x).to_degrees();

        Camera {
            target: position.clone(),
            position,
            aspect,
            fov,
            near,
            far,
            pitch,
            yaw,
        }
    }

    pub fn direction(&self) -> Vector {
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        Vector::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize()
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.pitch = (self.pitch + 360. + angle) % 360.;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.yaw = (self.yaw + 360. + angle) % 360.;
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn translate(&mut self, direction: Direction) {
        let movement = match direction {
            Direction::Forward => self.direction(),
            Direction::Backward => -self.direction(),
            Direction::Left => -self
                .direction()
                .cross(&Vector::new(0.0, 1.0, 0.0))
                .normalize(),
            Direction::Right => self
                .direction()
                .cross(&Vector::new(0.0, 1.0, 0.0))
                .normalize(),
        }
        .normalize()
            * 1.;

        self.target = self.position + movement;
    }

    pub fn update(&mut self) {
        let factor = 0.75;

        self.position.x = lerp(self.position.x, self.target.x, factor);
        self.position.y = lerp(self.position.y, self.target.y, factor);
        self.position.z = lerp(self.position.z, self.target.z, factor);
    }
}
