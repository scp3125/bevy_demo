use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Melee(pub Damage);

#[derive(Component)]
pub struct Gun(pub Bullet);

#[derive(Component)]
pub struct Bullet(pub Damage);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;
