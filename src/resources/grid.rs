use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::GridCoords;

#[derive(Debug, Resource)]
pub struct Grid {
    width: i32,
    height: i32,
    cells: HashMap<GridCoords, GridCell>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: HashMap::new(),
        }
    }

    pub fn get(&self, coords: GridCoords) -> Option<GridCell> {
        self.cells.get(&coords).copied()
    }

    pub fn set(&mut self, coords: GridCoords, cell: GridCell) {
        self.cells.insert(coords, cell);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell {
    Solid,
    Skip,
    Door(Entity),
    Npc(Entity),
    Player(Entity),
}
