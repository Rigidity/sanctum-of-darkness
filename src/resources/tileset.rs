use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Tileset {
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
}
