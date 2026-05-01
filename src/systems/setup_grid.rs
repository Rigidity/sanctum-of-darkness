use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{Grid, GridCell, Solid, TILE_SIZE};

pub fn setup_grid(
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
    solid_cells: Query<&GridCoords, With<Solid>>,
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

            for solid_cell in solid_cells.iter() {
                grid.set(*solid_cell, GridCell::Solid);
            }

            println!("Grid: {grid:#?}");

            commands.insert_resource(grid);
        }
    }
    Ok(())
}
