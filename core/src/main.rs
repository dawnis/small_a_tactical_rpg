mod logging;
mod hexagonaltile;
mod terrain;

use nannou::prelude::*;
use crate::logging::init_logging;
use lazy_static::lazy_static;
use log::*;
use structopt::StructOpt;
use core::Mrgb;
use hexboard::Board;
use hexboard::{TileFactory, Hextile};
use hexagonaltile::tile::HexagonalTile;
use hexagonaltile::factory::HextileFactory;
use std::path;


///Small, a tactical RPG Game
#[derive(StructOpt, Debug)]
#[structopt(name = "Small RPG")]
pub struct Opt {
    /// Set the color of the monster
    #[structopt(short, long, default_value = "lvl1_sprite")]
    lvl: String,
    /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , -vvvv or more: trace)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,
}

lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
}

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub board: Board<HexagonalTile>,
    pub edge_scale: f32,
    pub world_offset: (i32, i32)
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    //let image_pth = path::Path::new("/home/dawnis/git/small_a_tactical_rpg/assets/maps/lvl1_sprite.png");
    let edge_scale = 25.;
    let app_rect = app.window_rect();
    let htf = HextileFactory::new(None);
    let board = Board::default_board( (app_rect.left(), app_rect.right(), app_rect.top(), app_rect.bottom()));
    //let board = Board::from_img(image_pth, edge_scale, 
    //                           (app_rect.left(), app_rect.right(),
    //                            app_rect.top(), app_rect.bottom()));
    Model {
        _window,
        board,
        edge_scale,
        world_offset: (0, 0)
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {

    let speed = 2;
    let max_scale = 50.;
    let min_scale = 5.;

    if app.keys.down.contains(&Key::C) {
        model.world_offset = (0, 0)
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

    //if app.keys.down.contains(&Key::Plus) && model.edge_scale < max_scale {
    //    model.edge_scale += 1.;
    //    model.board = model.board.update_scale(model.edge_scale);
    //}

    //if app.keys.down.contains(&Key::Minus) && model.edge_scale > min_scale {
    //    model.edge_scale -= 1.;
    //    model.board = model.board.update_scale(model.edge_scale);
    //}

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let htf = HextileFactory::new(Some(&draw));
    htf.display_board(&model.board, model.world_offset);
    draw.background().color(BEIGE);

    draw.to_frame(app, &frame).unwrap();
}
