use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlayerEntrypoint {
    #[default]
    Spawner,
    Door(usize),
}
