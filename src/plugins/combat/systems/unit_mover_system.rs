use avian3d::{dynamics::rigid_body::LinearVelocity, physics_transform::Position};
use bevy::prelude::*;

use crate::plugins::components::UnitMover;

pub fn unit_mover_system(mut query: Query<(&UnitMover, &mut Position, &mut LinearVelocity)>) {
    for (unit_mover, position, linear_velocity) in query.iter_mut() {
        // todo
    }
}
