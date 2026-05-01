use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::GridCoords;

use crate::Direction;

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

    pub fn remove(&mut self, coords: GridCoords) {
        self.cells.remove(&coords);
    }

    pub fn move_entity(
        &mut self,
        current_coords: GridCoords,
        dir: Direction,
    ) -> Option<Vec<Movement>> {
        let cell = self.get(current_coords)?;

        let entity = match cell {
            GridCell::Player(entity) | GridCell::Npc(entity) | GridCell::Movable(entity) => entity,
            GridCell::Solid | GridCell::Skip | GridCell::Door(_) => {
                return None;
            }
        };

        let mut target_coords = translate_coords(current_coords, dir);

        while let Some(GridCell::Skip) = self.get(target_coords) {
            target_coords = translate_coords(target_coords, dir);
        }

        if target_coords.x >= self.width || target_coords.y >= self.height {
            return None;
        }

        match self.get(target_coords) {
            Some(GridCell::Skip) => unreachable!(),
            Some(GridCell::Player(_) | GridCell::Npc(_) | GridCell::Solid | GridCell::Door(_)) => {
                None
            }
            Some(GridCell::Movable(_)) => {
                let mut movements = self.move_entity(target_coords, dir)?;

                self.remove(current_coords);
                self.set(target_coords, cell);

                movements.push(Movement {
                    entity,
                    new_coords: target_coords,
                });

                Some(movements)
            }
            None => {
                self.remove(current_coords);
                self.set(target_coords, cell);

                Some(vec![Movement {
                    entity,
                    new_coords: target_coords,
                }])
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell {
    Solid,
    Skip,
    Movable(Entity),
    Door(Entity),
    Npc(Entity),
    Player(Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub entity: Entity,
    pub new_coords: GridCoords,
}

fn translate_coords(coords: GridCoords, dir: Direction) -> GridCoords {
    match dir {
        Direction::Up => coords + GridCoords::new(0, 1),
        Direction::Down => coords + GridCoords::new(0, -1),
        Direction::Left => coords + GridCoords::new(-1, 0),
        Direction::Right => coords + GridCoords::new(1, 0),
    }
}
