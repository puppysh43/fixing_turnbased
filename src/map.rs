use crate::prelude::*;
use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}
#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: IVec2) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    pub fn can_enter_tile(&self, point: IVec2) -> bool {
        self.in_bounds(point) && (self.tiles[map_idx(point.x, point.y)] == TileType::Floor)
    }
}
