use nannou::prelude::*;
use nannou::color::rgba;
use hex2d::{Coordinate, Position, Direction, Spacing};
use hexboard::GamePiece;
use crate::soots::arthropods::Arthropod;
use crate::OPT;

pub struct SootSprite {
    stype: Arthropod,
    position: Position,
    texture: wgpu::Texture,
    
}

impl SootSprite {

    pub fn new(app: &App, loc: (i32, i32), orient: Direction, stype: Arthropod) -> Self {
        SootSprite { 
            stype,
            position: Position::new(Coordinate::new(loc.0, loc.1), orient),
            texture: stype.to_texture(app),
        }
    }

    pub fn draw(&self, draw: &Draw, scale: f32, off: Coordinate) {
        let xy_c = self.position.coord + off;
        let xy = xy_c.to_pixel(Spacing::FlatTop(scale));
        let r_sze = (3.).sqrt() * scale;
        let bb = Rect::from_w_h(r_sze, r_sze);
        draw.texture(&self.texture)
            .wh(bb.wh())
            .xy(Vec2::new(xy.0, xy.1));


        if OPT.verbosity >= 2 {
            for c in self.legal_moves() {
                let c_off = c + off;
                let hexagon_pixel_ctr = c_off.to_pixel(Spacing::FlatTop(scale));

                let step = 60;

                let points = (0..=360).step_by(step).map(|i| {
                    let radian = deg_to_rad(i as f32);
                    let x = radian.cos() * scale + hexagon_pixel_ctr.0;
                    let y = radian.sin() * scale + hexagon_pixel_ctr.1;
                    (pt2(x, y), ROYALBLUE)
                });

                draw.polygon().points_colored(points);
            }
        }
    }

    pub fn legal_moves(&self) -> Vec<Coordinate> {
        let possible_moves = self.stype.moves(self.position);
        possible_moves.iter().map(|&x| x.coord).collect()
    }
}

impl GamePiece for SootSprite {

    fn position(&self) -> Position {
        self.position
    }

    fn walk(&mut self) {
        let new_position = self.position + Coordinate::new(0, 1);

    }
}
