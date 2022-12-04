mod tilefactory;

use nannou::prelude::*;
use hexboard::Board;
use tilefactory::HextileFactory;
use std::path;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub board: Board<HextileFactory>,
    pub edge_scale: f32,
    pub world_offset: (i32, i32)
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let image_pth = path::Path::new("/home/dawnis/git/small_a_tactical_rpg/assets/maps/lvl1_sprite.png");
    let edge_scale = 25.;
    let app_rect = app.window_rect();
    let board = Board::from_img(image_pth, edge_scale, 
                                (app_rect.left(), app_rect.right(),
                               app_rect.top(), app_rect.bottom()));
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

    if app.keys.down.contains(&Key::Plus) && model.edge_scale < max_scale {
        model.edge_scale += 1.;
        model.board = model.board.update_scale(model.edge_scale);
    }

    if app.keys.down.contains(&Key::Minus) && model.edge_scale > min_scale {
        model.edge_scale -= 1.;
        model.board = model.board.update_scale(model.edge_scale);
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    model.board.display(model.world_offset, &draw);

    draw.to_frame(app, &frame).unwrap();
}
