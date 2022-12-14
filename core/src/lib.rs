use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;

pub mod hexagonaltile;
pub mod soots;
pub mod factory;

///Type alias for nannou color type
pub type Mrgb = Rgb<Srgb, u8>;

