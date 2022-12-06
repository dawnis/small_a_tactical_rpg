//! Hexgametile 
//!
//! Implements the hexagonal tiles and their properties to be used with hexboard.
use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;

pub mod hexagon;
pub mod terrain;

///Type alias for nannou color type
pub type Mrgb = Rgb<Srgb, u8>;

