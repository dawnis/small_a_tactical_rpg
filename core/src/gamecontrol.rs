use hexboard::Board;
use nannou::prelude::*;
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use crate::soots::arthropods::Arthropod;
use crate::gamecontrol::movement::SpriteMovement;
use std::cell::RefCell;

mod movement;

pub struct GController {
    pub board: Board<HexagonalTile>,
    pub sprites: Vec<RefCell<SootSprite>>,
}

impl GController {

    pub fn new(board: Board<HexagonalTile>) -> Self {
        GController { 
            board,
            sprites: Vec::new(),
        }
    }

    pub fn place(&mut self, piece: SootSprite) {
        self.sprites.push(RefCell::new(piece));
    }

    pub fn update_bugs(&mut self, app: &App) {
        for sprite in self.sprites.iter().filter(|&s| !matches!(s.borrow().stype, Arthropod::Hero{name: _})) {
            if sprite.borrow().last_updated > sprite.borrow().stype.reaction_time() {
                sprite.borrow_mut().last_updated = 0.;
                self.walk(sprite);
            } else {
                sprite.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }
    }

    pub fn command_hero(&mut self, app: &App, hero_name: &str, command: usize) {
        for hero in self.sprites.iter().filter(|&s| s.borrow().stype == Arthropod::Hero{name: String::from(hero_name)}) {
            if hero.borrow().last_updated > hero.borrow().stype.reaction_time() {
                hero.borrow_mut().last_updated = 0.;
                self.command_move(hero, command);
            } else {
                hero.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }
    }

}
