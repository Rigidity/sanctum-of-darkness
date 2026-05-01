mod components;
mod resources;
mod state;
mod systems;

pub use components::*;
pub use resources::*;
pub use state::*;
pub use systems::*;

use bevy::{prelude::*, window::WindowResolution};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

pub const TILE_SIZE: IVec2 = IVec2::new(12, 12);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Sanctum of Darkness".to_string(),
                        resolution: WindowResolution::new(1600, 900),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PixelCameraPlugin)
        .add_plugins(LdtkPlugin)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<Tileset>(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .init_resource::<PlayerEntrypoint>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Identifier("Start_House".to_string()))
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        })
        .register_ldtk_entity::<SpawnerBundle>("Spawner")
        .register_ldtk_entity::<DoorBundle>("Door")
        .register_ldtk_entity::<NpcBundle>("NPC")
        .register_ldtk_int_cell::<SolidBundle>(1)
        .add_observer(spawn_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Transform::from_xyz(150.0, 80.0, 0.0),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap.ldtk").into(),
        ..default()
    });
}
