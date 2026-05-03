use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

pub fn translate_coords(coords: GridCoords, dir: Direction) -> GridCoords {
    match dir {
        Direction::Up => coords + GridCoords::new(0, 1),
        Direction::Down => coords + GridCoords::new(0, -1),
        Direction::Left => coords + GridCoords::new(-1, 0),
        Direction::Right => coords + GridCoords::new(1, 0),
    }
}
