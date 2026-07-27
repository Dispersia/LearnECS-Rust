//! Game library: your binary adds [`GamePlugin`]. Put your game setup
//! (systems, resources, observers) in here.
//!
//! # Components
//!
//! Write components anywhere in this library (any module, not
//! `main.rs`), deriving `Component, Reflect, Default` with
//! `#[reflect(Component, Default)]`. No registration code is needed;
//! Bevy's `reflect_auto_register` picks up the `Reflect` derive.

use bevy::prelude::*;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, _app: &mut App) {}
}

// Example: an editable component. Uncomment, save, and Rebuild to see
// `Health` in the inspector's Add Component list.
//
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}
