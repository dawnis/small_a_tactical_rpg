//! Hexboard
//!
//! Hexboard is a library for coordinating hexagonal tile tracking and display. 

pub mod builder;
pub mod gamecontrol;

use gamecontrol::GController;
use hex2d::{Spacing, Coordinate, Position};
use crate::builder::BoardBuilder;
use std::collections::BTreeMap;

/// Interface for hexagonal tiles
pub trait Hextile {
    fn default() -> Self;
    fn from_pixel(pixel: image::Rgba<u8>) -> Self;
}

pub trait GamePiece<H: Hextile> {
    fn position(&self) -> Position;
    fn moveset(&self) -> Vec<Coordinate>;
    fn is_legal(&self, t: &H) -> bool;
    fn walk(&mut self, move_set: Vec<Coordinate>);
}

/// Interface for the drawing api
pub trait TileFactory {
    type Tile: Hextile;
    type Sprite: GamePiece<Self::Tile>;
    fn draw_tile(&self, c: Coordinate, scale: f32, t: &Self::Tile);
    fn draw_sprite(&self, c: Coordinate, scale: f32, s: &Self::Sprite);
    fn display_board(&self, b: &Board<Self::Tile, Self::Sprite>, offset: (i32, i32));
}

#[derive(Default, Clone, Copy)]
struct ViewBoundary {
    left: f32,
    right: f32, 
    top: f32, 
    bottom: f32
}

/// Maps hexagonal tiles by their axial coordinate.
pub struct Board<H: Hextile, G: GamePiece<H>> {
    pub tiles: BTreeMap<Coordinate, H>,
    pub pieces: Vec<G>,
    scale: f32,
    vb: ViewBoundary,
}

impl<H: Hextile, G: GamePiece<H>> Board<H, G> {

    /// Determines if a coordinate is in the viewing window
    pub fn is_viewable(&self, cd: Coordinate) -> bool {
        let hpc = cd.to_pixel(Spacing::FlatTop(self.scale));
        self.vb.left < hpc.0 && self.vb.right > hpc.0 
           && self.vb.bottom < hpc.1  && self.vb.top >  hpc.1 
    }

    pub fn update_scale(&mut self, new_scale: f32) {
        self.scale = new_scale;
    }

    pub fn place(&mut self, new_piece: G) {
        self.pieces.push(new_piece);
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn builder() -> BoardBuilder<H, G> {
        BoardBuilder::new()
    }

    pub fn controller(&self) -> GController<H, G> {
        GController::new(self.tiles)
    }

}
