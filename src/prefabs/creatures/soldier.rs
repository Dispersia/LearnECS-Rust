use bevy::prelude::*;

use crate::plugins::components::*;

pub fn soldier(position: Vec3) -> impl Scene {
    bsn! {
        Faction::Friendly

        Health {
            current: 100,
            max: 100
        }

        Transform {
            translation: position
        }
    }
}
