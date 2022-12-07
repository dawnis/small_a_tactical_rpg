use std::marker::PhantomData;
use crate::{Board, Hextile, ViewBoundary};
use hex2d::{Coordinate, Spin, XY};
use std::collections::BTreeMap;


#[derive(Default)]
pub struct BoardBuilder<H: Hextile> {
    _tile: PhantomData<H>
}

impl<H: Hextile> BoardBuilder<H> {
    pub fn new() -> BoardBuilder<H> {
        BoardBuilder{_tile: PhantomData}
    }

    pub fn island_c(&self, num_layers: i32, app_window: (f32, f32, f32, f32)) -> Board<H> {
        assert!(num_layers > 0);
        let mut game_board = BTreeMap::new();

        let center = Coordinate::new(0, 0);
        game_board.insert(center, <H as Hextile>::default());


        for layer in 1..num_layers {

            if layer == 1 {
                for &c in &center.neighbors() {
                    game_board.insert(c, <H as Hextile>::default());
                }
            } else {
                for ring_c in center.ring_iter(layer, Spin::CCW(XY)) {
                    game_board.insert(ring_c, <H as Hextile>::default());
                }
            }
        }

        Board {tiles: game_board, scale: 25., vb: ViewBoundary{left: app_window.0,
                                                   right: app_window.1,
                                                   top: app_window.2,
                                                   bottom: app_window.3} }
    }
}
