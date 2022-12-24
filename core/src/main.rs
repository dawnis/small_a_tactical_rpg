mod logging;
mod hexagonaltile;
mod soots;
mod factory;

use nannou::prelude::*;
use crate::logging::init_logging;
use core::gamecontrol::GController;
use core::Mrgb;
use log::*;
use hexboard::*;
use hexboard::builder::BoardBuilder;
use core::factory::HextileFactory;
use std::path::Path;
use core::soots::sootsprite::SootSprite;
use core::soots::arthropods::Arthropod::*;
use core::{OPT, cfg_fetch};
use hex2d::Direction::*;

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub gctl: GController, 
    pub world_offset: (i32, i32),
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let level_cfg_key = "levels.".to_owned() + &OPT.lvl;
    let level = cfg_fetch(&level_cfg_key);
    debug!("Loading level: {:?}", level);

    let level_maps_folder = cfg_fetch("assets.maps");
    let image_pth = Path::new(&level_maps_folder).join(Path::new(&level));
    debug!("Image path read at {:?}", image_pth);

    let app_rect = app.window_rect();
    let app_rect_as_tuple = (app_rect.left(), app_rect.right(), app_rect.top(), app_rect.bottom());

    let board = match OPT.generate_method.as_str() {
        "image" => BoardBuilder::new().map_image_px(&image_pth, app_rect_as_tuple),
        "platform" => BoardBuilder::new().island_c(20, (app_rect.left(), app_rect.right(), app_rect.top(), app_rect.bottom())),
         _ => panic!("Unable to choose map generation option")
    };

    let mut gctl = GController::new(board);

    let wasp_vec: Vec<SootSprite> = (0..9).map(|_| SootSprite::new(app, (0, 0), YZ, Wasp{})).collect();

    for w in wasp_vec {
        gctl.place(w);
    }

    gctl.place(SootSprite::new(app, (-10, 0), ZY, Hero{name: String::from("jak")}));
    gctl.place(SootSprite::new(app, (-9, 0), ZY, Hero{name: String::from("mag")}));
    gctl.place(SootSprite::new(app, (-10, 1), ZY, Hero{name: String::from("sed")}));

    Model {
        _window,
        gctl,
        world_offset: (0, 0),
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {

    let speed = 2;
    let max_scale = 50.;
    let min_scale = 5.;

    model.gctl.update_bugs(app);

    let focus_hero = model.gctl.focus();

    if app.keys.down.contains(&Key::H) {
        model.gctl.change_focus(app);

        for hero in model.gctl.sprites.iter().filter(|&x| x.borrow().stype == Hero {name: model.gctl.focus()}) {
            let hpos = hero.borrow().position;
            model.world_offset = (-hpos.coord.x, -hpos.coord.y)
        }
    }

    // Turn left/right keys for Heros
    if app.keys.down.contains(&Key::Z) {
        model.gctl.command_hero(app, &focus_hero, 0);
    }

    if app.keys.down.contains(&Key::X) {
        model.gctl.command_hero(app, &focus_hero, 1);
    }

    // Move Keys for heros
    if app.keys.down.contains(&Key::S) {
        model.gctl.command_hero(app, &focus_hero, 2);
    }

    if app.keys.down.contains(&Key::D) {
        model.gctl.command_hero(app, &focus_hero, 3);
    }

    if app.keys.down.contains(&Key::E) {
        model.gctl.command_hero(app, &focus_hero, 4);
    }

    if app.keys.down.contains(&Key::W) {
        model.gctl.command_hero(app, &focus_hero, 5);
    }

    if app.keys.down.contains(&Key::Q) {
        model.gctl.command_hero(app, &focus_hero, 6);
    }

    if app.keys.down.contains(&Key::A) {
        model.gctl.command_hero(app, &focus_hero, 7);
    }

    if app.keys.down.contains(&Key::C) {
        for hero in model.gctl.sprites.iter().filter(|&x| x.borrow().stype == Hero {name: String::from("jak")}) {
            let hpos = hero.borrow().position;
            model.world_offset = (-hpos.coord.x, -hpos.coord.y)
        }
    }

    if app.keys.down.contains(&Key::Right) {
        model.world_offset = (model.world_offset.0 - speed, model.world_offset.1 + speed/2)
    }

    if app.keys.down.contains(&Key::Left) {
        model.world_offset = (model.world_offset.0 + speed, model.world_offset.1 - speed/2)
    }

    if app.keys.down.contains(&Key::Up) {
        model.world_offset = (model.world_offset.0, model.world_offset.1 + speed)
    }

    if app.keys.down.contains(&Key::Down) {
        model.world_offset = (model.world_offset.0, model.world_offset.1 - speed)
    }

    if app.keys.down.contains(&Key::Plus) && model.gctl.board.scale() < max_scale {
        let updated_scale = model.gctl.board.scale() + 1.;
        model.gctl.board.update_scale(updated_scale);
    }

    if app.keys.down.contains(&Key::Minus) && model.gctl.board.scale() > min_scale {
        let updated_scale = model.gctl.board.scale() - 1.;
         model.gctl.board.update_scale(updated_scale);
    }

    model.gctl.check_captures();
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();

    let htf = HextileFactory::new(Some(&draw));

    htf.display_board(&model.gctl.board, &model.gctl.sprites, model.world_offset);

    draw.background().color(BEIGE);

    draw.to_frame(app, &frame).unwrap();
}
