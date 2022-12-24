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

        let mut hero_moved = false;

        for hero in self.sprites.iter().filter(|&s| s.borrow().stype == Arthropod::Hero{name: String::from(hero_name)}) {
            if hero.borrow().last_updated > hero.borrow().stype.reaction_time() {
                hero.borrow_mut().last_updated = 0.;
                self.command_move(hero, command);
                hero_moved = true;
            } else {
                hero.borrow_mut().last_updated += app.duration.since_prev_update.ms();
            }
        }

        let mut hero_coordinate: Vec<Coordinate> = self.sprites.iter()
                .filter(|&s| s.borrow().stype == Arthropod::Hero{name: String::from(hero_name)})
                .map(|s| s.borrow().position.coord).collect();
        let hero_coordinate = hero_coordinate.pop().unwrap();


        //All valid sprites are kept
        for sprite in self.sprites.iter() {
            if (sprite.borrow().stype == Arthropod::Wasp{}) && ((hero_coordinate == sprite.borrow().position.coord) && hero_moved) {
                    sprite.borrow_mut().alive = false;
                }
            }
    }

    pub fn focus(&self) -> String {
        self.hfocus.clone()
    }

    pub fn change_focus(&mut self, app: &App) {
        if app.duration.since_start.ms() - self.last_hero_switch > 100. {
            if self.hfocus == *"sed" {
                self.hfocus = String::from("jak")
            } else if self.hfocus == *"jak" {
                self.hfocus = String::from("mag") 
            }
            else {
                self.hfocus = String::from("sed")
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
