use hex2d::Position;
use hexboard::Board;
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use crate::soots::arthropods::Arthropod;
use std::collections::BTreeMap;

pub struct GController {
    pub board: Board<HexagonalTile>,
    pub bugs: Vec<SootSprite>,
    pub heros: BTreeMap<String, SootSprite>,
}

impl GController {

    pub fn place(&mut self, new_piece: SootSprite) {
        match new_piece.stype {
            Arthropod::Hero{ name } => {
                self.heros.insert(name, new_piece);
            },
            _ => self.bugs.push(new_piece),
        };
    }


    pub fn new(board: Board<HexagonalTile>) -> Self {
        GController { 
            board,
            bugs: Vec::new(),
            heros: BTreeMap::new(),
        }
    }

    pub fn filter_move_set(&self, s: &SootSprite, requested: Vec<Position>) -> Vec<Position> {

        //1st make sure all requested coordinates are actually keyed
        let on_board_coordinates: Vec<Position> = requested.iter().cloned().filter(|&x| self.board.tiles
                                                                                   .get(&x.coord)
                                                                                   .is_some())
                                                                                   .collect();

        //2nd check if the tile at each coordinate is allowed by the walk_sprite
        let sprite_allowed_tiles: Vec<Position> = on_board_coordinates.iter()
                                                                      .cloned()
                                                                      .filter(|&x| s.legal_tile(self.board.tiles.get(&x.coord).unwrap())).collect();

        //return vec<coordinate> of allowed moves
        sprite_allowed_tiles
    }

    pub fn legal_moves(&self, s: &SootSprite) -> Vec<Position> {
        self.filter_move_set(s, s.moveset())
    }
}
