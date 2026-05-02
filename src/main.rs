mod assets;
mod components;
mod json5_asset;
mod plugins;
mod resources;
mod systems;

pub use assets::*;
pub use components::*;
pub use json5_asset::*;
pub use plugins::*;
pub use resources::*;
pub use systems::*;

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
                        resolution: WindowResolution::new(1600, 900),
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
        .add_systems(Update, move_player.run_if(in_state(LevelState::Playing)))
        .run();
}

fn setup_camera(mut commands: Commands) {
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
}
