use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct Selected {
    pub visual_entity: Entity,
    pub show_scale: f32,
}
