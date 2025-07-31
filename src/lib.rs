pub mod utils;
pub mod structures;

pub const EPSILON: f32 = 0.0001;

// structures
pub use structures::vector::Vector;
pub use structures::canvas::Canvas;
pub use structures::color::Color;
pub use structures::matrix::Matrix;
pub use structures::ray::Ray;
pub use structures::point::Point;

// parsing
pub mod parser;
pub use parser::{get_info_map, NewCanvas};

