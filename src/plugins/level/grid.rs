use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};

use crate::{Direction, Door, LevelState, Movable, Npc, Player, PlayerEntrypoint, TILE_SIZE};

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

pub fn setup_grid(
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
    player_entrypoint: Res<PlayerEntrypoint>,
    int_grid_cells: Query<(&IntGridCell, &GridCoords)>,
    door_cells: Query<(Entity, &GridCoords), With<Door>>,
    npc_cells: Query<(Entity, &GridCoords), With<Npc>>,
    movable_cells: Query<(Entity, &GridCoords), With<Movable>>,
    player_cells: Query<(Entity, &GridCoords), With<Player>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut next_state: ResMut<NextState<LevelState>>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .unwrap();
            let level = ldtk_project.get_raw_level_by_iid(level_iid.get()).unwrap();

            let mut grid = Grid::new(level.px_wid / TILE_SIZE.x, level.px_hei / TILE_SIZE.y);

            for (cell, coords) in int_grid_cells.iter() {
                let cell = match cell.value {
                    1 => GridCell::Solid,
                    2 => GridCell::Skip,
                    _ => continue,
                };
                grid.set(*coords, cell);
            }

            for (entity, coords) in door_cells.iter() {
                grid.set(*coords, GridCell::Door(entity));
            }

            for (entity, coords) in npc_cells.iter() {
                grid.set(*coords, GridCell::Npc(entity));
            }

            for (entity, coords) in movable_cells.iter() {
                grid.set(*coords, GridCell::Movable(entity));
            }

            for (entity, coords) in player_cells.iter() {
                let coords = match *player_entrypoint {
                    PlayerEntrypoint::Spawner => *coords,
                    PlayerEntrypoint::Door(_) => todo!(),
                };

                let translation = grid_coords_to_translation(coords, TILE_SIZE);

                commands.entity(entity).insert((
                    coords,
                    Transform::from_xyz(translation.x, translation.y, 0.0),
                ));

                grid.set(coords, GridCell::Player(entity));
            }

            commands.insert_resource(grid);

            next_state.set(LevelState::Playing);
        }
    }

    Ok(())
}
