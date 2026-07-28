use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct Target {
    pub target_entity: Option<Entity>,
}
