use nannou::prelude::*;
use crate::hexagonaltile::tile::HexagonalTile;
use rand::{thread_rng, Rng};
use hex2d::{Angle, Coordinate, Position, Direction, Spacing};
use hexboard::GamePiece;
use crate::soots::arthropods::Arthropod;
use crate::OPT;

pub struct SootSprite {
    pub stype: Arthropod,
    pub position: Position,
    pub last_updated: f64,
    texture: wgpu::Texture,
    pub alive: bool,
}

impl SootSprite {

    pub fn new(app: &App, loc: (i32, i32), orient: Direction, stype: Arthropod) -> Self {
        SootSprite { 
            stype: stype.clone(),
            position: Position::new(Coordinate::new(loc.0, loc.1), orient),
            last_updated: 0.0,
            texture: stype.to_texture(app),
            alive: true,
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
            for c in self.legal_next_coord() {
                let c_off = c + off;
                let hexagon_pixel_ctr = c_off.to_pixel(Spacing::FlatTop(scale));

                let step = 60;

                let points = (0..=360).step_by(step).map(|i| {
                    let radian = deg_to_rad(i as f32);
                    let x = radian.cos() * scale + hexagon_pixel_ctr.0;
                    let y = radian.sin() * scale + hexagon_pixel_ctr.1;
                    (pt2(x, y), ROYALBLUE)
                });

                draw.polyline().weight(3.0).points_colored(points);
            }
        }
    }

    pub fn legal_next_coord(&self) -> Vec<Coordinate> {

        let possible_moves = self.stype.moves(self.position);

        possible_moves.iter()
            .map(|&x| x.coord)
            .filter(|&c| c != self.position.coord)
            .collect()
    }
}

impl SootSprite {
    pub fn legal_tile(&self, tile: &HexagonalTile) -> bool {
        self.stype.is_legal_terrain(tile)
    }

    pub fn moveset(&self) -> Vec<Position> {
        self.stype.moves(self.position)
    }

    pub fn walk(&mut self, move_set: Vec<Position>) -> Coordinate {
        let mut rng = thread_rng();
        let m = rng.gen_range(0..move_set.len());
        self.position = move_set[m];
        self.position.coord
    }

    pub fn command(&mut self, legal: Vec<Position>, cmd: usize) -> Coordinate {
        let angular_change = self.position.dir - Direction::ZY;

        let ccw_shift = match angular_change {
            Angle::Forward => 0,
            Angle::Right => 1,
            Angle::RightBack => 2,
            Angle::Back => 3,
            Angle::LeftBack => 4,
            Angle::Left => 5,
        };

        let angular_command_corrected = if cmd < 2 {cmd}
        else {
            (((cmd - 2) + ccw_shift) % 6) + 2
        };

        let movements = &self.moveset();
        let requested = movements[angular_command_corrected];
        if legal.iter().filter(|&x| x == &requested).count() > 0 {
            self.position = requested;
        }

        self.position.coord
    }

}

impl GamePiece for SootSprite {

    fn position(&self) -> Position {
        self.position
    }

}
