use nannou::prelude::*;
use hexboard::Board;
use std::path;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    _window: window::Id,
    pub board: Board,
    pub edge_scale: f32,
    pub world_offset: (i32, i32)
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let image_pth = path::Path::new("maps/lvl1_sprite.png");
    let edge_scale = 25.;
    let board = Board::from_img(image_pth, edge_scale);
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
    let bounds = app.window_rect();
    draw.background().color(BEIGE);
    model.board.make(model.world_offset, &draw, bounds);

    draw.to_frame(app, &frame).unwrap();
}
