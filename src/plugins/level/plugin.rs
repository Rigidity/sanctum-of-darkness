use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    AssetState, DoorBundle, LevelState, LoadedAssets, MovableBundle, NpcBundle, PlayerBundle,
    PlayerEntrypoint, move_player, on_switch_level, setup_grid, trigger_interactions,
};

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
            .add_systems(OnEnter(AssetState::Ready), setup_ldtk_world)
            .add_systems(Update, setup_grid.run_if(in_state(LevelState::Loading)))
            .add_systems(
                Update,
                (move_player, trigger_interactions)
                    .chain()
                    .run_if(in_state(LevelState::Playing)),
            )
            .add_observer(on_switch_level);
    }
}

fn setup_ldtk_world(mut commands: Commands, loaded_assets: Res<LoadedAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: loaded_assets.tilemap.clone().into(),
        ..default()
    });

    commands.insert_resource(LevelSelection::Identifier("Start_House".to_string()));
}
