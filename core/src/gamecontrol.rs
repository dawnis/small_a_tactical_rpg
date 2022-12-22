use hexboard::Board;
use nannou::prelude::*;
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use crate::soots::arthropods::Arthropod;
use std::collections::BTreeMap;
use crate::gamecontrol::movement::SpriteMovement;
use std::cell::RefCell;

mod movement;

pub struct GController {
    pub board: Board<HexagonalTile>,
    pub bugs: Vec<RefCell<SootSprite>>,
    pub heros: BTreeMap<String, SootSprite>,
}

impl GController {

    pub fn new(board: Board<HexagonalTile>) -> Self {
        GController { 
            board,
            bugs: Vec::new(),
            heros: BTreeMap::new(),
        }
    }


    pub fn place(&mut self, new_piece: SootSprite) {
        match new_piece.stype {
            Arthropod::Hero{ref name } => {
                self.heros.insert(name.to_string(), new_piece);
            },
            _ => self.bugs.push(RefCell::new(new_piece)),
        };
    }

    pub fn update_bugs(&mut self, app: &App) {
        for sprite in self.bugs.iter() {
            if sprite.borrow().last_updated > sprite.borrow().stype.reaction_time() {
                sprite.borrow_mut().last_updated = 0.;
                self.walk(sprite);
            } else {
                sprite.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }
    }


}
