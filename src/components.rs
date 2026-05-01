use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub direction: Direction,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Door {
    pub room: String,
    pub target: u8,
    pub index: u8,
    pub id: Option<String>,
}

impl From<&EntityInstance> for Door {
    fn from(instance: &EntityInstance) -> Self {
        let room = instance
            .field_instances
            .iter()
            .find_map(|field| {
                if field.identifier == "Room"
                    && let FieldValue::String(value) = &field.value
                {
                    value.as_ref()
                } else {
                    None
                }
            })
            .unwrap();

        let target = instance
            .field_instances
            .iter()
            .find_map(|field| {
                if field.identifier == "Target"
                    && let FieldValue::Int(value) = &field.value
                {
                    value.as_ref()
                } else {
                    None
                }
            })
            .unwrap();

        let index = instance
            .field_instances
            .iter()
            .find_map(|field| {
                if field.identifier == "Index"
                    && let FieldValue::Int(value) = &field.value
                {
                    value.as_ref()
                } else {
                    None
                }
            })
            .unwrap();

        let id = instance.field_instances.iter().find_map(|field| {
            if field.identifier == "ID"
                && let FieldValue::String(value) = &field.value
            {
                value.as_ref()
            } else {
                None
            }
        });

        Self {
            room: room.clone(),
            target: *target as u8,
            index: *index as u8,
            id: id.cloned(),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[from_entity_instance]
    pub door: Door,
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Npc {
    pub id: String,
}
impl From<&EntityInstance> for Npc {
    fn from(instance: &EntityInstance) -> Self {
        let id = instance
            .field_instances
            .iter()
            .find_map(|field| {
                if field.identifier == "ID"
                    && let FieldValue::String(value) = &field.value
                {
                    value.as_ref()
                } else {
                    None
                }
            })
            .unwrap();

        Self { id: id.clone() }
    }
}
#[derive(Default, Bundle, LdtkEntity)]
pub struct NpcBundle {
    #[from_entity_instance]
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
