use bevy::prelude::*;

use crate::plugins::components::*;

pub fn zombie(position: Vec3) -> impl Scene {
    bsn! {
        Faction::Zombie

        Health {
            current: 100,
            max: 100
        }

        Transform {
            translation: position
        }

        FindTarget {
            range: 5.,
            target_faction: Faction::Friendly,
            timer_max: 0.2
        }

        Target

        UnitMover {
            move_speed: 5,
            rotation_speed: 10
        }

        Zombie
    }
}
