use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
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
        .add_plugin(EguiPlugin) // Add the plugin // Optionally define gravity
        .add_startup_system(init_camera)
        .add_startup_system(spwan_human)
        .add_startup_system(spwan_zombie)
        .add_system(move_player)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("中文");
    });
}
