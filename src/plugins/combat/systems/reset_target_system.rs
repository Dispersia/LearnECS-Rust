use bevy::prelude::*;

use crate::plugins::components::Target;

pub fn reset_target_system(query: Query<&Target>) {
    for target in query.iter() {
        // target
    }
}
