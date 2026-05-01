use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Spawner;

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpawnerBundle {
    pub spawner: Spawner,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Component)]
pub struct Player;
