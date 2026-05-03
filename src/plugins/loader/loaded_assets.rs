use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::NpcDef;

#[derive(AssetCollection, Resource)]
pub struct LoadedAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 12,
        tile_size_y = 12,
        columns = 206,
        rows = 50,
        padding_x = 1,
        padding_y = 1,
        offset_x = 1,
        offset_y = 1
    ))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "tileset.png")]
    pub sprite: Handle<Image>,
    #[asset(path = "npcs", collection(typed))]
    pub npcs: Vec<Handle<NpcDef>>,
    #[asset(path = "tilemap.ldtk")]
    pub tilemap: Handle<LdtkProject>,
}
