use bevy::prelude::*;
use std::fmt;

use crate::AppState;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Manager>()
            .init_resource::<Teams>()
            .add_system(spawn_manager.in_schedule(OnEnter(AppState::NewGame)));
    }
}

#[derive(Default, Component, Resource, Clone)]
pub struct Manager {
    pub name: String,
    pub team: Teams,
}

#[derive(PartialEq, Debug, Resource, Default, Copy, Clone)]
pub enum Teams {
    #[default]
    Texas,
    Oklahoma,
}

impl fmt::Display for Teams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn spawn_manager(mut commands: Commands) {
    commands.spawn(Manager {
        name: "".to_string(),
        team: Teams::Texas,
    });
}
