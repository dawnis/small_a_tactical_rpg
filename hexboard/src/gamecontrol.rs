use hex2d::Coordinate;
use crate::{Board, Hextile, GamePiece};

pub struct GController<H: Hextile, G: GamePiece> {
    gboard: Board<H, G>
}

impl<H: Hextile, G: GamePiece> GController<H, G> {
    pub fn new(gboard: Board<H, G>) -> Self {
        GController { gboard }
    }

    pub fn filter_move_set(&self, s: G, requested: Vec<Coordinate>) -> Vec<Coordinate> {
        vec![Coordinate::new(0,0)]
    }

    pub fn walk_sprite(&self, s: &mut G) {
        s.walk();
    }
}
