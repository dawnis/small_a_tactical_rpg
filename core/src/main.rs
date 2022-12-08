mod logging;
mod hexagonaltile;
mod terrain;

use nannou::prelude::*;
use crate::logging::init_logging;
use lazy_static::lazy_static;
use structopt::StructOpt;
use core::Mrgb;
use log::*;
use hexboard::*;
use hexboard::builder::BoardBuilder;
use hexagonaltile::{tile::HexagonalTile, factory::HextileFactory};
use std::path::Path;
use config::{Config, File, FileFormat};


///Small, a tactical RPG Game
#[derive(StructOpt, Debug)]
#[structopt(name = "Small RPG")]
pub struct Opt {
    /// Set the level that is loaded
    #[structopt(short, long, default_value = "treehouse")]
    lvl: String,
    /// Set whether board is loaded using generation or a map
    #[structopt(short, long, default_value = "image")]
    generate_method: String,
    /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , -vvvv or more: trace)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,
}

lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
    pub static ref CFG: Option<Config> = {
        let cfg = Config::builder().add_source(File::new("core/game_configuration.toml", FileFormat::Toml));

        match cfg.build() {
            Ok(config) => Some(config),
            Err(_) => {
                error!("couldn't load game configuration");
                None
                }
            }
        };

}

fn cfg_fetch(key: &str) -> String {
    CFG.as_ref().expect("Unable to generate configuration!")
       .get_string(key).unwrap_or_else(|_| panic!("Couldn't find requested configuration key: {}", key))
}

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub board: Board<HexagonalTile>,
    pub world_offset: (i32, i32)
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

    //let htf = HextileFactory::new(None);
    let board = match OPT.generate_method.as_str() {
        "image" => BoardBuilder::new().map_image_px(&image_pth, app_rect_as_tuple),
        "platform" => BoardBuilder::new().island_c(20, (app_rect.left(), app_rect.right(), app_rect.top(), app_rect.bottom())),
         _ => panic!("Unable to choose map generation option")
    };

    Model {
        _window,
        board,
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

    if app.keys.down.contains(&Key::Plus) && model.board.scale() < max_scale {
        let updated_scale = model.board.scale() + 1.;
        model.board.update_scale(updated_scale);
    }

    if app.keys.down.contains(&Key::Minus) && model.board.scale() > min_scale {
        let updated_scale = model.board.scale() - 1.;
         model.board.update_scale(updated_scale);
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let htf = HextileFactory::new(Some(&draw));
    htf.display_board(&model.board, model.world_offset);
    draw.background().color(BEIGE);

    draw.to_frame(app, &frame).unwrap();
}
