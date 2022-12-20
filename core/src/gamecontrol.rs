use hex2d::{Coordinate, Position};
use hexboard::Board;
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;

pub struct GController {
    pub board: Board<HexagonalTile, SootSprite>
}

impl GController {
    pub fn new(board: Board<HexagonalTile, SootSprite>) -> Self {
        GController { board }
    }

    pub fn filter_move_set(&self, s: &SootSprite, requested: Vec<Coordinate>) -> Vec<Coordinate> {

        //1st make sure all requested coordinates are actually keyed
        let on_board_coordinates: Vec<Coordinate> = requested.iter().cloned().filter(|&x| self.board.tiles.get(&x).is_some()).collect();

        //2nd check if the tile at each coordinate is allowed by the walk_sprite
        let sprite_allowed_tiles: Vec<Coordinate> = on_board_coordinates.iter().cloned().filter(|&x| s.is_legal(self.board.tiles.get(&x).unwrap())).collect();

        //return vec<coordinate> of allowed moves
        sprite_allowed_tiles
    }

    pub fn walk_sprite(&self, legal: Vec<Position>, s: &mut SootSprite) {
        s.walk(legal);
    }

    pub fn legal_moves(&self, s: &SootSprite) -> Vec<Coordinate> {
        let moves_requested = s.moveset().iter()
                                         .map(|&x| x.coord)
                                         .collect();

        self.filter_move_set(s, moves_requested)
    }
}
