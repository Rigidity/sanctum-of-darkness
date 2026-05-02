use bevy::prelude::*;

use crate::AssetState;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(AssetState = AssetState::Ready)]
pub enum LevelState {
    #[default]
    Loading,
    Playing,
}
