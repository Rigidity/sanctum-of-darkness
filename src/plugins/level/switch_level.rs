use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::LevelState;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlayerEntrypoint {
    #[default]
    Spawner,
    Door(u8),
}

#[derive(Event)]
pub struct SwitchLevel {
    pub room: String,
    pub target_door: u8,
}

pub fn on_switch_level(
    on: On<SwitchLevel>,
    mut level_selection: ResMut<LevelSelection>,
    mut player_entrypoint: ResMut<PlayerEntrypoint>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    *level_selection = LevelSelection::Identifier(on.room.clone());
    *player_entrypoint = PlayerEntrypoint::Door(on.target_door);

    next_state.set(LevelState::Loading);
}
