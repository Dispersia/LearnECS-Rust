pub mod components;
mod systems;

use systems::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                find_target_system,
                health_dead_test_system,
                reset_target_system,
                selected_visual_system,
                shoot_attack_system,
                testing_system,
                unit_mover_system,
            ),
        );
    }
}
