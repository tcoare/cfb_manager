use bevy::prelude::*;

use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .add_system(spawn_players.in_schedule(OnEnter(AppState::InGame)));
    }
}

#[derive(Default, Component, Resource, Clone)]
pub struct Player {
    pub name: String,
}

pub fn spawn_players(mut commands: Commands) {
    commands.spawn_batch(vec![
        (Player {
            name: "Arch Manning".to_string(),
        },),
        (Player {
            name: "Chase Wolf".to_string(),
        },),
    ]);
}
