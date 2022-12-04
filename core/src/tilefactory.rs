use hexgametile::hexagon::HexagonalTile;
use hexboard::TileFactory;
use nannou::prelude::*;

struct HexTileFactory {
    api: Draw,
}

impl TileFactory for HextileFactory {
    type Output =  HexagonalTile;

    fn api(&self) -> Draw {
        self.api
    }

    fn from_pixel(&self, scale: f32, pixel: Rgba<u8>) -> HexagonalTile {
        HexagonalTile::from_pixel(self.api, edge, pixel)
    }

    fn build(&self) -> HexagonalTile {
        HexagonalTile::new(self.api, 25., Terrain::Air)
    }

}
