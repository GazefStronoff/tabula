use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};
use systems::{camera_update, spawn_camera};

#[path ="../player/systems.rs"]
mod player_systems;

mod systems;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_update.after(player_systems::spawn_player));
    }
}
