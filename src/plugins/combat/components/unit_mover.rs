use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct UnitMover {
    pub move_speed: i32,
    pub rotation_speed: i32,
}
