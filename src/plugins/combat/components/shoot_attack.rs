use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct ShootAttack {
    pub timer: f32,
    pub timer_max: f32,
}
