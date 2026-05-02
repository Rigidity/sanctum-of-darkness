use bevy::prelude::*;

use crate::{LoadedAssets, NpcDef, Registry};

pub fn setup_registry(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
    npc_assets: Res<Assets<NpcDef>>,
) {
    let mut registry = Registry::new();

    for npc in &loaded_assets.npcs {
        registry.insert_npc(npc_assets.get(npc).unwrap().clone());
    }

    commands.insert_resource(registry);
}
