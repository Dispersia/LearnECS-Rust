mod plugins;
mod prefabs;

use plugins::CombatPlugin;
use prefabs::prelude::*;

use bevy::prelude::*;

pub fn create_app() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run()
}

#[derive(Default)]
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatPlugin);
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.queue_spawn_scene(zombie(Vec3::new(0., 0., 0.)));

    commands.queue_spawn_scene(soldier(Vec3::new(0., 0., 0.)));
}
