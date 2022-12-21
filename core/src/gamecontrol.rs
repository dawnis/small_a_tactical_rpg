use hex2d::{Coordinate, Position};
use hexboard::{GamePiece, Board};
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use std::collections::BTreeMap;

pub struct GController<'a> {
    board: &'a mut Board<HexagonalTile, SootSprite>,
}

impl<'a> GController<'a> {

    pub fn tiletree(&self) -> &BTreeMap<Coordinate, HexagonalTile> {
        self.board.get_tiles()
    }

    pub fn chars(&self) -> & Vec<SootSprite> {
        self.board.get_pieces()
    }

    pub fn mchars(&mut self) -> &Vec<SootSprite> {
        self.board.get_mut_pieces()
    }

    pub fn new(board: &'a mut Board<HexagonalTile, SootSprite>) -> Self {
        GController { 
            board,
        }
    }

    pub fn filter_move_set(&self, s: &SootSprite, requested: Vec<Position>) -> Vec<Position> {

        //1st make sure all requested coordinates are actually keyed
        let on_board_coordinates: Vec<Position> = requested.iter().cloned().filter(|&x| self.tiletree()
                                                                                   .get(&x.coord)
                                                                                   .is_some())
                                                                                   .collect();

        //2nd check if the tile at each coordinate is allowed by the walk_sprite
        let sprite_allowed_tiles: Vec<Position> = on_board_coordinates.iter()
                                                                      .cloned()
                                                                      .filter(|&x| s.legal_tile(self.tiletree().get(&x.coord).unwrap())).collect();

        //return vec<coordinate> of allowed moves
        sprite_allowed_tiles
    }

    pub fn walk_sprite(&mut self, legal: Vec<Position>, s: &mut SootSprite) {
        s.walk(legal);
    }

    pub fn legal_moves(&self, s: &SootSprite) -> Vec<Position> {
        self.filter_move_set(s, s.moveset())
    }
}