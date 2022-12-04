use nannou::prelude::*;
use crate::terrain::Terrain;
use hex2d::{Coordinate, Spacing};
use hexboard::Hextile;

/// HexagonalTile stores the scale and properties of each game tile.
#[derive(Debug)]
pub struct HexagonalTile {
    api: Draw,
    edge: f32,
    pub terrain: Terrain,
}

impl HexagonalTile {
    pub fn new(api: Draw, edge: f32, terrain: Terrain) -> Self {
        HexagonalTile {
            api,
            edge,
            terrain,
        }
    }

    pub fn from_pixel(api: Draw, edge: f32, pixel: image::Rgba<u8>) -> Self {
        HexagonalTile::new(api, edge, Terrain::from(pixel))
    }
}

impl Hextile for HexagonalTile {

    fn get_scale(&self) -> f32 {
        self.edge
    }

    fn resize(&self, new_edge_size: f32) -> Self {
        HexagonalTile::new(self.api, new_edge_size, self.terrain)
    }

    fn draw(&self, axial: Coordinate) {

        let draw = &self.api;

        let hexagon_pixel_ctr = axial.to_pixel(Spacing::FlatTop(self.edge));

        let step = 60;
        let points = (0..=360).step_by(step).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos() * self.edge + hexagon_pixel_ctr.0;
            let y = radian.sin() * self.edge + hexagon_pixel_ctr.1;
            (pt2(x, y), self.terrain.color())
        });
        draw.polygon().points_colored(points);
        let points = (0..=360).step_by(step).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos() * self.edge + hexagon_pixel_ctr.0;
            let y = radian.sin() * self.edge + hexagon_pixel_ctr.1;
            (pt2(x, y), BLACK)
        });
        draw.polyline().weight(1.0).points_colored(points);
    }


}
