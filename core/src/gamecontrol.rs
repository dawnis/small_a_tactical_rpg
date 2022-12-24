use hexboard::Board;
use nannou::prelude::*;
use hex2d::Coordinate;
use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use crate::soots::arthropods::Arthropod;
use crate::gamecontrol::movement::SpriteMovement;
use std::cell::RefCell;

mod movement;

enum GameState {
    Won,
    Lost,
    Ongoing,
}

pub struct GController {
    pub board: Board<HexagonalTile>,
    pub sprites: Vec<RefCell<SootSprite>>,
    gamestate: GameState,
    hfocus: String,
    last_hero_switch: f64,
}

impl GController {

    pub fn new(board: Board<HexagonalTile>) -> Self {
        GController { 
            board,
            sprites: Vec::new(),
            gamestate: GameState::Ongoing,
            hfocus: String::from("sed"),
            last_hero_switch: 0.,
        }
    }

    pub fn place(&mut self, piece: SootSprite) {
        self.sprites.push(RefCell::new(piece));
    }

    pub fn update_bugs(&mut self, app: &App) {
        let mut xc: Vec<Coordinate> = Vec::new();
        for sprite in self.sprites.iter().filter(|&s| !matches!(s.borrow().stype, Arthropod::Hero{name: _})) {
            if sprite.borrow().last_updated > sprite.borrow().stype.reaction_time() {
                sprite.borrow_mut().last_updated = 0.;
                xc.push(self.walk(sprite));
            } else {
                sprite.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }

        //Check if any of the new coordinates match the heros
        for hero in self.sprites.iter().filter(|&s| matches!(s.borrow().stype, Arthropod::Hero{name: _})) {
            if xc.iter().filter(|&c| c == &hero.borrow().position.coord).count() > 0 {
                hero.borrow_mut().alive = false;
            }
        }
    }

    pub fn command_hero(&mut self, app: &App, hero_name: &str, command: usize) {
        let mut hc: Vec<Coordinate> = Vec::new();
        for hero in self.sprites.iter().filter(|&s| s.borrow().stype == Arthropod::Hero{name: String::from(hero_name)}) {
            if hero.borrow().last_updated > hero.borrow().stype.reaction_time() {
                hero.borrow_mut().last_updated = 0.;
                hc.push(self.command_move(hero, command));
            } else {
                hero.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }

        //All valid sprites are kept
        if !hc.is_empty() {
            let hero_coordinate = hc.pop().unwrap();
            for sprite in self.sprites.iter() {
                if (sprite.borrow().stype == Arthropod::Wasp{}) && (hero_coordinate == sprite.borrow().position.coord) {
                        sprite.borrow_mut().alive = false;
                    }
                }
        }
    }

    pub fn focus(&self) -> String {
        self.hfocus.clone()
    }

    pub fn change_focus(&mut self, app: &App) {
        if app.duration.since_start.ms() - self.last_hero_switch > 100. {
            let mut names: Vec<String> = Vec::new();
            for sprite in self.sprites.iter() {
                if let Arthropod::Hero{name} = &sprite.borrow().stype {
                    names.push(name.to_string());
                }
            };

            if names.is_empty() {
                return;
            }

            let h_index = names.iter().position(|h| h == &self.hfocus);

            match h_index {
                Some(idx) => {
                    let next = (idx + 1) % names.len();
                    self.hfocus = names[next].clone();
                },
                None => self.hfocus = names[0].clone(),
            }
            self.last_hero_switch = app.duration.since_start.ms();
        }
    }

    ///Checks dead/alive status of all sprites -- only removes one per cycle
    pub fn check_captures(&mut self) {
        let dead_idx = self.sprites.iter().position(|s| !s.borrow().alive);
        if let Some(idx) = dead_idx {
                self.sprites.remove(idx);
        }
    }

}
