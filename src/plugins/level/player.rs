use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::{prelude::*, utils::grid_coords_to_translation};

use crate::{Direction, Door, Grid, GridCell, Player, SwitchLevel, TILE_SIZE, translate_coords};

pub fn move_player(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Sprite, &GridCoords, &mut Direction), With<Player>>,
) -> Result {
    let (mut sprite, &grid_coords, mut player_direction) = player_query.single_mut()?;

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

    *player_direction = direction;

    for movement in grid.move_entity(grid_coords, direction).unwrap_or_default() {
        let translation = grid_coords_to_translation(movement.new_coords, TILE_SIZE);

        commands.entity(movement.entity).insert((
            movement.new_coords,
            Transform::from_xyz(translation.x, translation.y, 0.0),
        ));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum PossibleInteraction {
    Door(Entity),
}

pub fn trigger_interactions(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grid: Res<Grid>,
    player_query: Query<(&GridCoords, &Direction, &Sprite), With<Player>>,
    door_query: Query<&Door>,
) -> Result {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return Ok(());
    }

    let (grid_coords, facing_direction, sprite) = player_query.single()?;

    let mut possible_interactions = HashMap::new();

    for direction in Direction::all() {
        let target_coords = translate_coords(*grid_coords, direction);

        if let Some(GridCell::Door(entity)) = grid.get(target_coords) {
            possible_interactions.insert(direction, PossibleInteraction::Door(entity));
        }
    }

    let interaction = if possible_interactions.len() == 1 {
        possible_interactions.into_values().next()
    } else {
        possible_interactions.get(facing_direction).copied()
    };

    match interaction {
        Some(PossibleInteraction::Door(entity)) => {
            let door = door_query.get(entity)?;

            commands.trigger(SwitchLevel {
                room: door.room.clone(),
                target_door: door.target,
                direction: *facing_direction,
                flip_x: sprite.flip_x,
            });
        }
        None => {}
    }

    Ok(())
}
