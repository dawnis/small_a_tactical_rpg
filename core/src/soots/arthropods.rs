use nannou::prelude::*;
use hex2d::{Coordinate, Direction, Position};
use hex2d::Direction::*;
use hex2d::Angle::{Left, Right};
use crate::cfg_fetch;
use std::path::Path;

fn step(stps: i32, d: Direction) -> Coordinate {
    match d {
        ZY => Coordinate::new( 0, stps),
        XZ => Coordinate::new( stps, -stps ),
        XY => Coordinate::new( stps, 0 ),
        YZ => Coordinate::new( 0, -stps),
        ZX => Coordinate::new( -stps, stps),
        YX => Coordinate::new( -stps, 0),
    }
}

/// arthropods define enemy types
#[derive(Debug, Clone, Copy)]
pub enum Arthropod {
    Wasp,
}

/// Picks up the texture for each type

impl Arthropod {

    pub fn to_texture(self, app: &App) -> wgpu::Texture {
        let asset_pth = cfg_fetch("assets.sprites");
        let texture_path = Path::new(&asset_pth).join(Path::new(&cfg_fetch(&self.to_config())));
        wgpu::Texture::from_path(app, texture_path).unwrap()
    }

    pub fn moves(&self, p: Position) -> Vec<Position> {
        match self {
            Arthropod::Wasp => {
                vec![
                    p + step(1, p.dir),
                    p + step(1, p.dir + Right),
                    p + step(1, p.dir + Left),
                ]
            }
        }
    }

    fn to_config(self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp => "wasp",
            _ => "none"
        }

    }

}
