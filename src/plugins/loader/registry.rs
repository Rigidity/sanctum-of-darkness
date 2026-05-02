use bevy::{platform::collections::HashMap, prelude::*};

use crate::{LoadedAssets, NpcDef};

#[derive(Debug, Default, Clone, Resource)]
pub struct Registry {
    npcs: HashMap<String, NpcDef>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_npc(&mut self, npc: NpcDef) {
        let id = npc.id.clone();
        self.npcs.insert(id, npc);
    }

    pub fn npc(&self, id: &str) -> &NpcDef {
        self.npcs.get(id).unwrap()
    }
}

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
