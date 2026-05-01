use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};

use crate::{Door, Grid, GridCell, Npc, Player, PlayerEntrypoint, TILE_SIZE};

pub fn setup_grid(
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
    player_entrypoint: Res<PlayerEntrypoint>,
    int_grid_cells: Query<(&IntGridCell, &GridCoords)>,
    door_cells: Query<(Entity, &GridCoords), With<Door>>,
    npc_cells: Query<(Entity, &GridCoords), With<Npc>>,
    player_cells: Query<(Entity, &GridCoords), With<Player>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
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
        }
    }
    Ok(())
}
