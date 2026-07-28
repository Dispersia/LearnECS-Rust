use bevy::prelude::*;

use crate::plugins::components::{ShootAttack, Target};

pub fn shoot_attack_system(mut query: Query<(&Target, &mut ShootAttack)>) {
    for (target, shoot_attack) in query.iter_mut() {
        // todo
    }
}
