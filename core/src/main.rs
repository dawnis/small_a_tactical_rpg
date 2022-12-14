mod logging;
mod hexagonaltile;
mod soots;
mod factory;

use nannou::prelude::*;
use crate::logging::init_logging;
use core::Mrgb;
use log::*;
use hexboard::*;
use hexboard::builder::BoardBuilder;
use hexagonaltile::tile::HexagonalTile;
use factory::HextileFactory;
use std::path::Path;

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub board: Board<HexagonalTile>,
    pub world_offset: (i32, i32),
    wasp: wgpu::Texture
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let level_cfg_key = "levels.".to_owned() + &OPT.lvl;
    let level = cfg_fetch(&level_cfg_key);
    debug!("Loading level: {:?}", level);

    let level_maps_folder = cfg_fetch("assets.maps");
    let image_pth = Path::new(&level_maps_folder).join(Path::new(&level));
    debug!("Image path read at {:?}", image_pth);

    let sprite_assets_path = cfg_fetch("assets.sprites");
    let wasp_sprite = cfg_fetch("sprites.wasp");
    let wasp_pth = Path::new(&sprite_assets_path).join(Path::new(&wasp_sprite));
    let wasp = wgpu::Texture::from_path(app, wasp_pth).unwrap();

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
        world_offset: (0, 0),
        wasp
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

    let r = Rect::from_w_h(model.board.scale(), model.board.scale());
    draw.texture(&model.wasp)
         .wh(r.wh());

    draw.to_frame(app, &frame).unwrap();
}
