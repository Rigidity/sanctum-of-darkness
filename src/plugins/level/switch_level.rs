use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{Direction, LevelState};

#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlayerEntrypoint {
    #[default]
    Spawner,
    Door {
        target_door: u8,
        direction: Direction,
        flip_x: bool,
    },
}

#[derive(Event)]
pub struct SwitchLevel {
    pub room: String,
    pub target_door: u8,
    pub direction: Direction,
    pub flip_x: bool,
}

pub fn on_switch_level(
    on: On<SwitchLevel>,
    mut level_selection: ResMut<LevelSelection>,
    mut player_entrypoint: ResMut<PlayerEntrypoint>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    *level_selection = LevelSelection::Identifier(on.room.clone());
    *player_entrypoint = PlayerEntrypoint::Door {
        target_door: on.target_door,
        direction: on.direction,
        flip_x: on.flip_x,
    };

    next_state.set(LevelState::Loading);
}
