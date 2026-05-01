use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Spawner;

#[derive(Debug, Default, Clone, Copy, Bundle, LdtkEntity)]
pub struct SpawnerBundle {
    pub spawner: Spawner,
    #[grid_coords]
    pub grid_coords: GridCoords,
}
