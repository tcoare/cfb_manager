use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use main_menu::MainMenuPlugin;
use systems::spawn_camera;

mod main_menu;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
