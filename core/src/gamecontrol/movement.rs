use hex2d::Position;
use crate::{gamecontrol::GController, soots::sootsprite::SootSprite};
use std::cell::RefCell;

pub trait SpriteMovement {
    fn legal_moves(&self, s: &SootSprite) -> Vec<Position>;
    fn filter_move_set(&self, s: &SootSprite, requested: Vec<Position>) -> Vec<Position>;
    fn walk(&self, s: &RefCell<SootSprite>);
    fn command_move(&self, s: &RefCell<SootSprite>);
}

impl SpriteMovement for GController {

    fn walk(&self, s: &RefCell<SootSprite>) {
        let legal_move_set = self.legal_moves(&s.borrow());
        s.borrow_mut().walk(legal_move_set);
    }

    fn command_move(&self, s: &RefCell<SootSprite>) {
        let legal_move_set = self.legal_moves(&s.borrow());
        s.borrow_mut().command(legal_move_set);
    }

    fn legal_moves(&self, s: &SootSprite) -> Vec<Position> {
        self.filter_move_set(s, s.moveset())
    }

   fn filter_move_set(&self, s: &SootSprite, requested: Vec<Position>) -> Vec<Position> {

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
}
