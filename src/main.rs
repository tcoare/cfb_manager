use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod manager;
mod menu;
mod player;

use manager::ManagerPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "College Football Manager".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(EguiPlugin)
        .add_startup_system(spawn_camera)
        .add_system(configure_visuals)
        .add_state::<AppState>()
        .add_plugin(MenuPlugin)
        .add_plugin(ManagerPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    NewGame,
    InGame,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn configure_visuals(mut egui_ctx: EguiContexts) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}
