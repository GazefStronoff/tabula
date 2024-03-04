use bevy::app::{Plugin, Update};

mod systems;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, systems::camera_update);
    }
}
