use bevy::prelude::*;

use crate::plugins::components::Faction;

#[derive(Component, FromTemplate)]
pub struct Unit {
    faction: Faction,
}
