use bevy::prelude::*;

use crate::plugins::components::Health;

pub fn health_dead_test_system(query: Query<&Health>) {
    for health in query.iter() {
        // health.current
    }
}
