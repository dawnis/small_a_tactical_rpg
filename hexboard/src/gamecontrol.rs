use std::collections::BTreeMap;
use std::marker::PhantomData;
use hex2d::Coordinate;
use crate::{Hextile, GamePiece};

pub struct GController<H: Hextile, G: GamePiece<H>> {
    tiles: BTreeMap<Coordinate, H>,
    _piece: PhantomData<G>,
}

impl<H: Hextile, G: GamePiece<H>> GController<H, G> {
    pub fn new(tiles: BTreeMap<Coordinate, H>) -> Self {
        GController {tiles, _piece: PhantomData}
    }

    pub fn filter_move_set(&self, s: G, requested: Vec<Coordinate>) -> Vec<Coordinate> {

        //1st make sure all requested coordinates are actually keyed
        let on_board_coordinates: Vec<&Coordinate> = requested.iter().filter(|&x| self.tiles.get(x).is_some()).collect();

        //2nd check if the tile at each coordinate is allowed by the walk_sprite
        let sprite_allowed_tiles = on_board_coordinates.iter().filter(|&x| s.is_legal(self.tiles.get(x)?)).collect();
        //return vec<coordinate> of allowed moves

        sprite_allowed_tiles
    }

    pub fn walk_sprite(&self, s: &mut G) {
        let moves_requested = s.moveset();
        let allowed_moveset = self.filter_move_set(s, moves_requested);
        s.walk(allowed_moveset);
    }
}
