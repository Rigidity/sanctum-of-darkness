mod grid;
mod level_state;

pub use grid::*;
pub use level_state::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{DoorBundle, MovableBundle, NpcBundle, PlayerBundle, PlayerEntrypoint};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_sub_state::<LevelState>()
            .init_resource::<PlayerEntrypoint>()
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                ..default()
            })
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<NpcBundle>("NPC")
            .register_ldtk_entity::<MovableBundle>("Movable")
            .insert_resource(LevelSelection::Identifier("Start_House".to_string()))
            .add_systems(OnEnter(LevelState::Loading), setup_ldtk_world)
            .add_systems(Update, setup_grid.run_if(in_state(LevelState::Loading)));
    }
}

fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap.ldtk").into(),
        ..default()
    });
}
