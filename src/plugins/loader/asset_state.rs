use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AssetState {
    #[default]
    Loading,
    Ready,
}
