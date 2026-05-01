use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};

use crate::{Direction, Grid, Player, TILE_SIZE};

pub fn move_player(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Sprite, &GridCoords), With<Player>>,
) -> Result {
    let (mut sprite, &grid_coords) = player_query.single_mut()?;

    let up_pressed = keyboard_input.just_pressed(KeyCode::KeyW);
    let down_pressed = keyboard_input.just_pressed(KeyCode::KeyS);
    let left_pressed = keyboard_input.just_pressed(KeyCode::KeyA);
    let right_pressed = keyboard_input.just_pressed(KeyCode::KeyD);

    let direction = if up_pressed && !down_pressed {
        Direction::Up
    } else if down_pressed && !up_pressed {
        Direction::Down
    } else if left_pressed && !right_pressed {
        Direction::Left
    } else if right_pressed && !left_pressed {
        Direction::Right
    } else {
        return Ok(());
    };

    match direction {
        Direction::Left => sprite.flip_x = true,
        Direction::Right => sprite.flip_x = false,
        _ => {}
    }

    for movement in grid.move_entity(grid_coords, direction).unwrap_or_default() {
        let translation = grid_coords_to_translation(movement.new_coords, TILE_SIZE);
        commands.entity(movement.entity).insert((
            movement.new_coords,
            Transform::from_xyz(translation.x, translation.y, 0.0),
        ));
    }

    Ok(())
}
