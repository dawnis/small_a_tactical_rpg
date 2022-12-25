use crate::Mrgb;
use image::Rgba;
use nannou::prelude::*;

/// Terrain sets the properties of each tile. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Rock,
    Veg,
    Air,
    Earth,
    Wood,
    Void,
}

/// This converts pixels in an image into terrain tiles. 
impl<T: std::cmp::PartialOrd<u8>> From<image::Rgba<T>> for Terrain {
    fn from(pix: Rgba<T>) -> Self {
        if pix.0[0] == 0 && pix.0[1] == 0 && pix.0[2] == 0 {
            Terrain::Void
        }
        else if pix.0[0] == 95 && pix.0[1] == 227 && pix.0[2] == 255 {
            Terrain::Air
        }
        else if pix.0[0] == 89 && pix.0[1] == 204 && pix.0[2] == 9 {
            Terrain::Veg
        }
        else if pix.0[0] == 143 && pix.0[1] == 86 && pix.0[2] == 59 {
            Terrain::Wood
        }
        else if pix.0[0] == 255 && pix.0[1] == 130 && pix.0[2] == 3 {
            Terrain::Earth
        }
        else {
            Terrain::Rock
        }
    }
}

impl Terrain {
    /// Controls how each terrain is represented using the Nannou palette. 
    pub fn color(&self) -> Mrgb {
        match self {
            Terrain::Rock => SLATEGRAY,
            Terrain::Air => DEEPSKYBLUE,
            Terrain::Veg => GREENYELLOW,
            Terrain::Earth => GOLDENROD,
            Terrain::Wood => SIENNA,
            _ => BLACK,
        }
    }
}
