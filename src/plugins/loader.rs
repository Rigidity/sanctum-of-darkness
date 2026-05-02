mod asset_state;
mod registry;

pub use asset_state::*;
pub use registry::*;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{Json5AssetLoader, LoadedAssets, NpcDef};

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetState>()
            .init_asset::<NpcDef>()
            .init_asset_loader::<Json5AssetLoader<NpcDef>>()
            .add_loading_state(
                LoadingState::new(AssetState::Loading)
                    .continue_to_state(AssetState::Ready)
                    .load_collection::<LoadedAssets>(),
            )
            .add_systems(OnEnter(AssetState::Ready), setup_registry);
    }
}
