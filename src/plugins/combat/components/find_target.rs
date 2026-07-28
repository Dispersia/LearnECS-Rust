use bevy::prelude::*;

use crate::plugins::components::Faction;

#[derive(Component, FromTemplate)]
pub struct FindTarget {
    pub range: f32,
    pub target_faction: Faction,
    pub timer: f32,
    pub timer_max: f32,
}
