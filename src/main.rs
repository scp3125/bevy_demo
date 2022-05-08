use bevy::prelude::*;
use heron::prelude::*;
use system::*;

mod asset;
mod bundle;
mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(init_camera)
        .add_startup_system(spwan_human)
        .add_startup_system(spwan_zombie)
        .add_system(move_player)
        .run();
}
