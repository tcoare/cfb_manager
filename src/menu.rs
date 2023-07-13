use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::{
    manager::{Manager, Teams},
    player::Player,
    AppState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_banner.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(menu_banner.in_set(OnUpdate(AppState::NewGame)))
            .add_system(main_menu.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(new_game.in_set(OnUpdate(AppState::NewGame)))
            .add_system(home_menu.in_set(OnUpdate(AppState::InGame)));
    }
}

fn menu_banner(mut contexts: EguiContexts) {
    egui::TopBottomPanel::top("top").show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("College Football Manager");
        });
    });
}

fn main_menu(
    mut contexts: EguiContexts,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        let screen_height = ui.ctx().available_rect().height();
        ui.allocate_space(egui::Vec2::new(0.0, screen_height / 2.5));

        ui.vertical_centered(|ui| {
            if ui.button("New Game").clicked() {
                // Switch to the new game state.
                println!("New Game");
                next_state.set(AppState::NewGame);
            }

            if ui.button("Load Game").clicked() {
                // Switch to the load game state.
                println!("Load Game");
            }

            if ui.button("Quit").clicked() {
                // Quit the game.
                println!("Quiting");
                exit.send(AppExit);
            }
        });
    });
}

fn new_game(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut query: Query<&mut Manager>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        // TODO learn how to properly lay things out
        let screen_height = ui.ctx().available_rect().height();
        ui.allocate_space(egui::Vec2::new(0.0, screen_height / 2.5));

        let mut manager = query.single_mut();
        ui.columns(3, |columns| {
            columns[1].vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut manager.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Team to manage: ");
                    egui::ComboBox::new("", "")
                        .selected_text(manager.team.to_string())
                        .show_ui(ui, |ui| {
                            for team in [Teams::Texas, Teams::Oklahoma] {
                                // The first parameter is a mutable reference to allow the choice to be modified when the user selects
                                // something else. The second parameter is the actual value of the option (to be compared with the currently)
                                // selected one to allow egui to highlight the correct label. The third parameter is the string to show.
                                ui.selectable_value(&mut manager.team, team, team.to_string());
                            }
                        });
                });
                if ui.button("Start Game").clicked() {
                    // Switch to the new game state.
                    println!("Starting new game");
                    println!("Creating Manager");
                    println!("{:?}, {:?}", manager.name, manager.team);
                    next_state.set(AppState::InGame);
                }
            });
        });
    });
}

fn home_menu(
    mut contexts: EguiContexts,
    mut exit: EventWriter<AppExit>,
    mut manager: Query<&Manager>,
    mut players: Query<&Player>,
) {
    let manager = manager.single_mut();
    egui::TopBottomPanel::top("top").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            };
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.label(manager.name.to_owned())
        })
    });

    egui::SidePanel::left("").show(contexts.ctx_mut(), |ui| {
        ui.label(manager.team.to_string());
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label("Welcome");

        for player in players.iter_mut() {
            ui.label(player.name.to_string());
        }
    });
}
