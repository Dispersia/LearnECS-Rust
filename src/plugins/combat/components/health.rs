use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
