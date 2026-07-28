use avian3d::physics_transform::Position;
use bevy::prelude::*;

use crate::plugins::components::{FindTarget, Target};

pub fn find_target_system(
    time: Res<Time>,
    mut query: Query<(&Position, &mut FindTarget, &mut Target)>,
) {
    for (pos, mut find_target, mut target) in query.iter_mut() {
        find_target.timer -= time.delta_secs();

        if find_target.timer > 0. {
            continue;
        }

        find_target.timer = find_target.timer_max;
    }
}
