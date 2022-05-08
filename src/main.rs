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
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/wqy-microhei.ttc")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    egui_context.ctx_mut().set_fonts(fonts);
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("这是中文");
    });
}
