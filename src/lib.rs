pub mod utils;
pub mod structures;
pub mod enums;

pub const EPSILON: f32 = 0.001;

// structures
pub use structures::vector::Vector;
pub use structures::camera::Camera;
pub use structures::canvas::Canvas;
pub use structures::color::Color;
pub use structures::material::Material;
pub use structures::light::Light;
pub use structures::matrix::Matrix;
pub use structures::ray::Ray;
pub use structures::point::Point;
pub use structures::object::Object;
pub use structures::object::Transform;
pub use structures::intersection::Intersection;
pub use structures::intersection::Intersect;
pub use structures::intersection::LocalIntersection;
pub use structures::shapes::shape::Shape;
pub use structures::shapes::sphere::Sphere;
pub use structures::shapes::cube::Cube;
pub use structures::shapes::disk::Disk;
pub use structures::shapes::torus::Torus;
pub use structures::shapes::triangle::Triangle;
pub use structures::world::World;
pub use structures::pattern::{Pattern, Axis};

// parsing
pub mod parser;
pub use parser::{get_info_map, NewCanvas};

pub use structures::render::Renderer;

pub use enums::directions::Direction;
pub use enums::keys::Key;