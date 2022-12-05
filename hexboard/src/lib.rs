//! Hexboard
//!
//! Hexboard is a library for coordinating hexagonal tile tracking and display. 

use hex2d::Spacing;
use hex2d::Coordinate;
use image::GenericImageView;
use nannou::prelude::*;
use std::path;
use std::collections::BTreeMap;
use std::rc::Rc;

/// Trait which must be implemented by tiles using this libary.
pub trait Hextile {
    fn get_scale(&self) -> f32;
    fn draw(&self, c: Coordinate);
}

/// Factory pattern implementation for tile builders
pub trait TileFactory {
    type Output: Hextile;
    //fn from_pixel(&self, scale: f32, pixel: image::Rgba<u8>) -> Self::Output;
    fn build(&self) -> Self::Output;
    //fn rescale(&self, tile: Box<dyn Hextile>, scale: f32) -> Self::Output;
    fn draw_tile(&self, t: Self::Output);
}

#[derive(Default, Clone, Copy)]
struct ViewBoundary {
    left: f32,
    right: f32, 
    top: f32, 
    bottom: f32
}

/// Maps hexagonal tiles by their axial coordinate.
pub struct Board<T: TileFactory> {
    pub tiles: BTreeMap<Coordinate, T::Output>,
    vb: ViewBoundary,
    tf: T,
}

impl<T: TileFactory> Board<T> {

    /// Determines if a coordinate is in the viewing window
    fn is_viewable(&self, cd: Coordinate, scale: f32) -> bool {
        let hpc = cd.to_pixel(Spacing::FlatTop(scale));
        self.vb.left < hpc.0 && self.vb.right > hpc.0 
           && self.vb.bottom < hpc.1  && self.vb.top >  hpc.1 
    }

    pub fn default_board(tf: T, app_window: (f32, f32, f32, f32)) -> Self {
        let mut game_board = BTreeMap::new();
        game_board.insert(Coordinate::new(0, 0), tf.build());
        Board {tf, tiles: game_board, vb: ViewBoundary::default()}
    }

    /// Draws the board using nannou.
    pub fn display(&self, offset: (i32, i32)) {
        for (loc, tile) in self.tiles.iter() {
            let oc = *loc + Coordinate::new(offset.0, offset.1);
            if self.is_viewable(oc, tile.get_scale()) {
                    self.tf.draw_tile(*tile);
               }
        }
    }

}
