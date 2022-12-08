use nannou::prelude::*;
use crate::terrain::Terrain;
use hex2d::{Coordinate, Spacing};
use hexboard::Hextile;

/// HexagonalTile stores the scale and properties of each game tile.
#[derive(Debug, Copy, Clone)]
pub struct HexagonalTile {
    pub terrain: Terrain,
}

impl HexagonalTile {
    pub fn new(terrain: Terrain) -> Self {
        HexagonalTile {
            terrain,
        }
    }

    pub fn draw(&self, draw: &Draw, axial: Coordinate, scale: f32) {

        let hexagon_pixel_ctr = axial.to_pixel(Spacing::FlatTop(scale));

        let step = 60;
        let points = (0..=360).step_by(step).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos() * scale + hexagon_pixel_ctr.0;
            let y = radian.sin() * scale + hexagon_pixel_ctr.1;
            (pt2(x, y), self.terrain.color())
        });
        draw.polygon().points_colored(points);
        let points = (0..=360).step_by(step).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos() * scale + hexagon_pixel_ctr.0;
            let y = radian.sin() * scale + hexagon_pixel_ctr.1;
            (pt2(x, y), BLACK)
        });
        draw.polyline().weight(1.0).points_colored(points);
    }
}

impl Hextile for HexagonalTile {

    fn default() -> Self {
        HexagonalTile::new(Terrain::Wood)
    }

    fn from_pixel(pixel: image::Rgba<u8>) -> Self {
        HexagonalTile::new(Terrain::from(pixel))
    }

}
