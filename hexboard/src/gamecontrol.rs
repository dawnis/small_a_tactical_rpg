use std::collections::BTreeMap;
use std::marker::PhantomData;
use hex2d::Coordinate;
use crate::{Hextile, GamePiece};

pub struct GController<'a, H: Hextile, G: GamePiece<H>> {
    tiles: &'a BTreeMap<Coordinate, H>,
    _piece: PhantomData<G>,
}

impl<'a, H: Hextile, G: GamePiece<H>> GController<'a, H, G> {
    pub fn new(tiles: &'a BTreeMap<Coordinate, H>) -> Self {
        GController {tiles, _piece: PhantomData}
    }

    pub fn filter_move_set(&self, s: &G, requested: Vec<Coordinate>) -> Vec<Coordinate> {

        //1st make sure all requested coordinates are actually keyed
        let on_board_coordinates: Vec<Coordinate> = requested.iter().cloned().filter(|&x| self.tiles.get(&x).is_some()).collect();

        //2nd check if the tile at each coordinate is allowed by the walk_sprite
        let sprite_allowed_tiles: Vec<Coordinate> = on_board_coordinates.iter().cloned().filter(|&x| s.is_legal(self.tiles.get(&x).unwrap())).collect();

        //return vec<coordinate> of allowed moves
        sprite_allowed_tiles
    }

    pub fn walk_sprite(&self, legal: Vec<Coordinate>, s: &mut G) {
        s.walk(legal);
    }

    pub fn legal_moves(&self, s: &G) -> Vec<Coordinate> {
        let moves_requested = s.moveset();
        self.filter_move_set(s, moves_requested)
    }
}
