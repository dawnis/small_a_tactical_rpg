use nannou::prelude::*;
use hex2d::{Coordinate, Position, Direction};
use hexboard::GamePiece;

pub struct SootSprite {
    texture: wgpu::Texture,
    position: Position,
}

impl SootSprite {
    pub fn new(texture: wgpu::Texture) -> Self {
        SootSprite { 
            texture,
            position: Position::new(Coordinate::new(0, 0), Direction::YZ),
        }
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
