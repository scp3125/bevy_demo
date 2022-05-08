use crate::component::*;
use bevy::math::const_vec3;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Bundle)]
pub struct Human {
    pub health: Health,
    pub weapon: Gun,
    pub velocity: Velocity,
    pub speed: Speed,
    player: Player,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    rotation_constraints: RotationConstraints,

    #[bundle]
    sprite: SpriteBundle,
}

impl Human {
    pub fn new() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 255.0, 0.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                ..default()
            },
            health: Health(100),
            weapon: Gun(Bullet(Damage(10))),
            velocity: Velocity {
                linear: Vec3::ZERO,
                angular: AxisAngle::default(),
            },
            speed: Speed(30000.0),
            player: Player,
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: 5.0 },
            rotation_constraints: RotationConstraints::lock(),
        }
    }
}

#[derive(Bundle)]
pub struct Zombie {
    pub health: Health,
    pub weapon: Melee,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,

    #[bundle]
    sprite: SpriteBundle,
}

impl Zombie {
    pub fn new() -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(255.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform {
                    translation: const_vec3!([20.0, 1.0, 0.0]),
                    ..default()
                },
                ..default()
            },
            health: Health(20),
            weapon: Melee(Damage(1)),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: 5.0 },
        }
    }
}
