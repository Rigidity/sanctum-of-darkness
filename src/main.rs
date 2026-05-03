mod plugins;

pub use plugins::*;

use bevy::{prelude::*, window::WindowResolution};
use bevy_modern_pixel_camera::prelude::*;

pub const TILE_SIZE: IVec2 = IVec2::new(12, 12);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Sanctum of Darkness".to_string(),
                        resolution: WindowResolution::new(1280, 720),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PixelCameraPlugin)
        .add_plugins((LoaderPlugin, LevelPlugin))
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Transform::from_xyz(150.0, 84.0, 0.0),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));
}
