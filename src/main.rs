mod components;

pub use components::*;

use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

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
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Identifier("Start_House".to_string()))
        .register_ldtk_entity::<SpawnerBundle>("Spawner")
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
