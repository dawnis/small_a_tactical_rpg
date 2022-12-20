use nannou::prelude::*;
use crate::hexagonaltile::terrain::{Terrain, Terrain::*};
use hex2d::{Coordinate, Direction, Position};
use hex2d::Direction::*;
use hex2d::Angle::{Left, Right};
use crate::cfg_fetch;
use hexboard::Hextile;
use crate::hexagonaltile::tile::HexagonalTile;
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

struct BugFormation {
    name: String,
    terrains: Vec<Terrain>,
    reaction: f64,
    vision: u32,
}

impl BugFormation {
    pub fn new( bug: &Arthropod ) -> Self {
        match bug {
            Arthropod::Wasp {reaction, vision} => BugFormation { 
                name: String::from("wasp"), 
                terrains: vec![Air, Wood, Earth],
                reaction: *reaction,
                vision: *vision, 
            }
        }
    }
}

/// arthropods define enemy types
#[derive(Debug, Clone, Copy)]
pub enum Arthropod {
    Wasp{reaction: f64, vision: u32},
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
            Arthropod::Wasp {reaction: _, vision: _}=> {
                vec![
                    p,
                    p + Left,
                    p + Right,
                    p + step(1, p.dir),
                    p + step(1, p.dir + Right) + Right,
                    p + step(1, p.dir + Left) + Left,
                ]
            }
        }
    }

    pub fn is_legal_terrain(&self, t: &HexagonalTile) -> bool {
        let bf = BugFormation::new(self);
        bf.terrains.iter().filter(|&tx| *tx == t.terrain).count() > 0
        }

    pub fn reaction_time(&self) -> f64 {
        match self {
            Arthropod::Wasp { reaction, vision: _ } => *reaction,
        }
    }

    fn to_config(self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp { reaction: _, vision: _ } => "wasp",
        }

    }

}
