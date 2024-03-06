use bevy::prelude::*;
use player_components::Player;

#[path = "../player/components.rs"]
mod player_components;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_update(
    mut player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation = player_transform.translation;
}
