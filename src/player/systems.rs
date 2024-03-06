use bevy::prelude::*;

use self::{
    components::Player,
    entity_components::{
        AnimationIndices, AnimationTimer, Direction, Facing, Health, MovementVelocity, Velocity,
    },
};

#[path = "./components.rs"]
pub mod components;

#[path = "../entity/components.rs"]
pub mod entity_components;

const PLAYER_SPEED: f32 = 1000.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/characters/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Health(5.0),
        Velocity(Vec3::ZERO),
        MovementVelocity(Vec3::ZERO),
        Facing(Direction::EAST),
    ));
}

pub fn player_movement(
    mut player_query: Query<(&mut MovementVelocity, &mut Facing), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, mut facing) = player_query.single_mut();

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        facing.0 = Direction::WEST;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        facing.0 = Direction::EAST;
    }

    velocity.0 = direction.normalize_or_zero() * PLAYER_SPEED;
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut player_query: Query<(&mut Sprite, &MovementVelocity, &Facing), With<Player>>,
) {
    let (mut sprite, velocity, facing) = player_query.single_mut();

    if velocity.0 == Vec3::ZERO {
        for (_, _, mut atlas) in &mut query {
            atlas.index = 0;
        }
    } else {
        for (indices, mut timer, mut atlas) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }

    match facing.0 {
        Direction::EAST => sprite.flip_x = false,
        Direction::WEST => sprite.flip_x = true,
        _ => (),
    }
}
