use bevy::prelude::*;

use crate::plugins::components::Selected;

pub fn selected_visual_system(query: Query<&Selected>) {
    for selected in query.iter() {
        // selected
    }
}
