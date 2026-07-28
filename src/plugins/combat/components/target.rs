use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct Target {
    target_entity: Option<Entity>,
}
