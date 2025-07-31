pub mod utils;
pub mod structures;

pub const EPSILON: f32 = 0.0001;

// structures
pub use structures::vector::Vector;
pub use structures::canvas::Canvas;
pub use structures::color::Color;
pub use structures::material::Material;
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

// parsing
pub mod parser;
pub use parser::{get_info_map, NewCanvas};

