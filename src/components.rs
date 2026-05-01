use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
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

#[derive(Default, Component)]
pub struct Movable;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MovableBundle {
    pub movable: Movable,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}
