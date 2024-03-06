use bevy::prelude::*;

pub enum Direction {
    //NORTH,
    EAST,
    //SOUTH,
    WEST,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Facing(pub Direction);

//Knockback, projectile, etc.
#[derive(Component)]
pub struct Velocity(pub Vec3);

//Movement
#[derive(Component)]
pub struct MovementVelocity(pub Vec3);

//TODO: verander
#[derive(Component)]
pub struct Stick(pub Vec3);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

//global animation loop/timer?
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
