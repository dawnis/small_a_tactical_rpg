use nannou::prelude::*;
use crate::hexagonaltile::terrain::{Terrain, Terrain::*};
use hex2d::{Coordinate, Direction, Position};
use hex2d::Direction::*;
use hex2d::Angle::{Left, Right};
use crate::cfg_fetch;
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

struct BugFormation<'a> {
    name: &'a str,
    terrains: Vec<Terrain>,
    reaction: f64,
    vision: u32,
}

impl<'a> BugFormation<'a> {
    pub fn new( bug: &'a Arthropod ) -> Self {
        match bug {
            Arthropod::Wasp {} => BugFormation { 
                name: "wasp", 
                terrains: vec![Air, Wood, Earth],
                reaction: 200.,
                vision: 4u32, 
            },
            Arthropod::Hero { name } => BugFormation { 
                name,
                terrains: vec![Wood, Earth, Veg, Void],
                reaction: 200.,
                vision: 9u32, 
            }
        }
    }
}

/// Athropods enum defines types of sprites in the game
/// Wasp are enemy NPCs
/// Hero are PCs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arthropod {
    Wasp{},
    Hero{name: String},
}

/// Picks up the texture for each type

impl Arthropod {

    pub fn to_texture(&self, app: &App) -> wgpu::Texture {
        let asset_pth = cfg_fetch("assets.sprites");
        let texture_path = Path::new(&asset_pth).join(Path::new(&cfg_fetch(&self.to_config())));
        wgpu::Texture::from_path(app, texture_path).unwrap()
    }

    pub fn moves(&self, p: Position) -> Vec<Position> {
        match self {
            Arthropod::Wasp{}=> {
                vec![
                    p,
                    p + Left,
                    p + Right,
                    p + step(1, p.dir),
                    p + step(1, p.dir + Right) + Right,
                    p + step(1, p.dir + Left) + Left,
                ]
            }
            Arthropod::Hero{name} => {
                let mut tf = vec![p + Left, p + Right];
                let mut mv: Vec<Position> = p.coord.neighbors()
                                                      .map(|x| Position::new(x, p.dir))
                                                      .to_vec();
                tf.append(&mut mv);
                tf
            }
        }
    }

    pub fn is_legal_terrain(&self, t: &HexagonalTile) -> bool {
        let bf = BugFormation::new(self);
        bf.terrains.iter().filter(|&tx| *tx == t.terrain).count() > 0
        }

    pub fn reaction_time(&self) -> f64 {
        let bf = BugFormation::new(self);
        bf.reaction
    }

    fn to_config(&self) -> String {
        "sprites.".to_owned() + match self {
            Arthropod::Wasp {} => "wasp",
            Arthropod::Hero {name} => name,
            }
    }
}
