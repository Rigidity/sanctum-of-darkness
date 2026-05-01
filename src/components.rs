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

#[derive(Default, Component)]
pub struct Solid;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct SolidBundle {
    pub solid: Solid,
}

#[derive(Default, Component)]
pub struct Door;

#[derive(Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    pub door: Door,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub struct Npc;

#[derive(Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    pub npc: Npc,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}
