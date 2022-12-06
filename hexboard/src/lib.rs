//! Hexboard
//!
//! Hexboard is a library for coordinating hexagonal tile tracking and display. 

mod builder;

use hex2d::Spacing;
use hex2d::Coordinate;
use crate::builder::BoardBuilder;
use image::GenericImageView;
use nannou::prelude::*;
use std::path;
use std::collections::BTreeMap;

/// Trait which must be implemented by tiles using this libary.
pub trait Hextile {
    fn get_scale(&self) -> f32;
    fn build() -> Self;
}

/// Factory pattern implementation for tile builders
pub trait TileFactory {
    type Output: Hextile;
    //fn from_pixel(&self, scale: f32, pixel: image::Rgba<u8>) -> Self::Output;
    //fn rescale(&self, tile: Box<dyn Hextile>, scale: f32) -> Self::Output;
    fn draw_tile(&self, c: Coordinate, t: &Self::Output);

    fn display_board(&self, b: &Board<Self::Output>, offset: (i32, i32));
}

#[derive(Default, Clone, Copy)]
struct ViewBoundary {
    left: f32,
    right: f32, 
    top: f32, 
    bottom: f32
}

/// Maps hexagonal tiles by their axial coordinate.
pub struct Board<H: Hextile> {
    pub tiles: BTreeMap<Coordinate, H>,
    vb: ViewBoundary,
}

impl<H: Hextile> Board<H> {
    /// Determines if a coordinate is in the viewing window
    pub fn is_viewable(&self, cd: Coordinate, scale: f32) -> bool {
        let hpc = cd.to_pixel(Spacing::FlatTop(scale));
        self.vb.left < hpc.0 && self.vb.right > hpc.0 
           && self.vb.bottom < hpc.1  && self.vb.top >  hpc.1 
    }

    pub fn builder() -> BoardBuilder<H> {
        BoardBuilder::default()
    }


}
