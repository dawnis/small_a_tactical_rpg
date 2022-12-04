use hexgametile::hexagon::HexagonalTile;
use nannou::prelude::*;

trait Tile {
    fn api() -> Draw;
}
trait TileFactory {
    type Output: Tile;
    fn build(&self) -> Self::Output;
}

struct HextileFactory {}

impl TileFactory for HextileFactory {
    type Output =  HexagonalTile;
    fn build(&self) -> HexagonalTile {
        HexagonalTile::default()
    }
}
