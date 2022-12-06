use std::marker::PhantomData;
use crate::{Board, Hextile, ViewBoundary};
use hex2d::{Coordinate, Spin, XY};
use std::collections::BTreeMap;


#[derive(Default)]
pub struct BoardBuilder<H: Hextile> {
    _tile: PhantomData<H>
}

impl<H: Hextile> BoardBuilder<H> {
    fn new() -> BoardBuilder<H> {
        BoardBuilder{_tile: PhantomData}
    }

    fn default_board(app_window: (f32, f32, f32, f32)) -> Board<H> {
        let mut game_board = BTreeMap::new();
        game_board.insert(Coordinate::new(0, 0), <H as Hextile>::build());
        Board {tiles: game_board, vb: ViewBoundary{left: app_window.0,
                                                   right: app_window.1,
                                                   top: app_window.2,
                                                   bottom: app_window.3} }
    }
}
