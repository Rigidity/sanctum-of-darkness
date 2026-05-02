use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Json5Extension;

#[derive(Debug, Default, Clone, Serialize, Deserialize, TypePath, Asset)]
pub struct NpcDef {
    pub id: String,
    pub name: String,
    pub dialogue: Dialogue,
}

impl Json5Extension for NpcDef {
    const EXTENSION: &'static str = "npc.json5";
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub lines: Vec<String>,
}
