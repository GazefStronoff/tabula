use bevy::prelude::*;
use systems::{animate_player, player_movement, spawn_player};

#[path ="./systems.rs"]
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (animate_player, player_movement));
    }
}
