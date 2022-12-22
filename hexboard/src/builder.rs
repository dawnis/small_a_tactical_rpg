use std::marker::PhantomData;
use crate::{Board, Hextile, ViewBoundary};
use hex2d::{Coordinate, Spin, XY};
use std::collections::BTreeMap;
use std::path::Path;
use image;
use image::GenericImageView;


#[derive(Default, Copy, Clone)]
pub struct BoardBuilder<H: Hextile> {
    _tile: PhantomData<H>,
    scale: f32,
}

impl<H: Hextile> BoardBuilder<H> {
    pub fn new() -> BoardBuilder<H> {
        BoardBuilder{_tile: PhantomData, scale: 25.}
    }

    pub fn map_image_px(&self, pixel_file: &Path, app_window: (f32, f32, f32, f32)) -> Board<H> {
        let (width, height) = image::image_dimensions(pixel_file).unwrap();

        let mut cx: Vec<(Coordinate, image::Rgba<u8>)> = Vec::new();

        let img = image::open(pixel_file).expect("file not found");

        for pixel in img.pixels() {
            let (x, y, c) = pixel;
            let x_c = x as i32 - width as i32 / 2;
            let y_c = y as i32 - height as i32 / 4 - x as i32 / 2;
           
            let hxc = Coordinate::new(x_c, y_c);
            cx.push((hxc, c));
        }

        Board {
            tiles: Self::pix2bmap(cx),
            vb: ViewBoundary { left: app_window.0, right: app_window.1, top: app_window.2, bottom: app_window.3 },
            scale: self.scale,
        }
    }

    fn pix2bmap(loc_pix_tuple: Vec<(Coordinate, image::Rgba<u8>)>) -> BTreeMap<Coordinate, H> {
        let mut game_board = BTreeMap::new();

        for pixel in loc_pix_tuple.iter() {
            game_board.insert(pixel.0, <H as Hextile>::from_pixel(pixel.1)); 
        }
        
        game_board
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

        Board {tiles: game_board, 
               scale: self.scale, 
               vb: ViewBoundary{left: app_window.0,
                                                   right: app_window.1,
                                                   top: app_window.2,
                                                   bottom: app_window.3},
        }
    }
}
