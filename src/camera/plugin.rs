use bevy::{app::{Plugin, Update}, ecs::schedule::IntoSystemConfigs};

mod systems;
#[path ="../main.rs"]
mod main;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, systems::camera_update.after(main::setup));
    }
}
