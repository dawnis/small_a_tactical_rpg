use nannou::prelude::*;
use hex2d::{Coordinate, Position, Direction};
use hexboard::Sprite;

pub struct Wasp {
    texture: wgpu::Texture,
    position: Position,
}

impl Wasp {
    pub fn new(texture: wgpu::Texture) -> Self {
        Wasp { 
            texture,
            position: Position::new(Coordinate::new(0, 0), Direction::YZ),
        }
    }
}

impl Sprite for Wasp {

    fn position(&self) -> Position {
        self.position
    }

    fn moves(&self) -> Vec<Coordinate> {
        let mut movement = Vec::new();

        for c in self.position.coord.neighbors() {
            movement.push(c);
        }

        movement
    }

    fn walk(&mut self) {
        let new_position = self.position + Coordinate::new(0, 1);

    }
}
