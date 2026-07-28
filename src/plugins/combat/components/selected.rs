use bevy::prelude::*;

#[derive(Component, FromTemplate)]
pub struct Selected {
    visual_entity: Entity,
    show_scale: f32,
}
