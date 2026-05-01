use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, utils::grid_coords_to_translation};

use crate::{Player, Spawner, TILE_SIZE};

pub fn spawn_player(
    on: On<Add, Spawner>,
    mut commands: Commands,
    query: Query<&GridCoords, With<Spawner>>,
) -> Result<()> {
    let spawner_coords = query.get(on.entity)?;

    let translation = grid_coords_to_translation(*spawner_coords, TILE_SIZE);

    commands.spawn((
        Player,
        Transform::from_xyz(translation.x, translation.y, 0.0),
        *spawner_coords,
    ));

    Ok(())
}
