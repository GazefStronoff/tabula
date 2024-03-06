use bevy::prelude::*;

use self::components::{MovementVelocity, Velocity};

#[path = "./components.rs"]
mod components;

pub fn entity_movement_update(
    time: Res<Time>,
    mut entity_query: Query<(&MovementVelocity, &mut Transform), With<MovementVelocity>>,
) {
    let Ok((velocity, mut transform)) = entity_query.get_single_mut() else {
        return;
    };
    transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
}

pub fn object_position_update(
    time: Res<Time>,
    mut object_query: Query<(&Velocity, &mut Transform), With<Velocity>>,
) {
    let Ok((velocity, mut transform)) = object_query.get_single_mut() else {
        return;
    };
    transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
}
