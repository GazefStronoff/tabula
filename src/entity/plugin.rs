use bevy::prelude::*;
use systems::{entity_movement_update, object_position_update};

mod systems;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (entity_movement_update, object_position_update));
    }
}
