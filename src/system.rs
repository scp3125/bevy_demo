use bevy::prelude::*;
use heron::prelude::*;

use crate::bundle::*;
use crate::component::*;

// 生成人类
pub fn spwan_human(mut commands: Commands) {
    commands.spawn_bundle(Human::new());
}

// 生成僵尸
pub fn spwan_zombie(mut commands: Commands) {
    commands.spawn_bundle(Zombie::new());
}

// 初始化摄像头
pub fn init_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// 移动玩家
pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Speed, &Player, &mut Velocity)>,
) {
    for (speed, _, mut velocity) in query.iter_mut() {
        let mut x = 0.0;
        let mut y = 0.0;
        if keys.pressed(KeyCode::D) {
            x += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            x -= 1.0;
        }
        if keys.pressed(KeyCode::W) {
            y += 1.0;
        }
        if keys.pressed(KeyCode::S) {
            y -= 1.0
        }
        if x != 0.0 || y != 0.0 {
            let direction = Vec3::from((x, y, 0.0)).normalize() * speed.0 * time.delta_seconds();
            // transform.translation += direction * speed.0 * time.delta_seconds();
            velocity.linear = direction;
        } else if velocity.linear != Vec3::ZERO {
            velocity.linear = Vec3::ZERO;
        }
    }
}

// 射击
pub fn shoot(
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&Gun, &Player)>,
) {
    if buttons.just_pressed(MouseButton::Left) {}
}
