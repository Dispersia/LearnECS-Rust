use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct ShootAttack {
    timer: f32,
    timer_max: f32,
}
