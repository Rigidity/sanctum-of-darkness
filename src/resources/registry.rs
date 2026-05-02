use bevy::{platform::collections::HashMap, prelude::*};

use crate::NpcDef;

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
