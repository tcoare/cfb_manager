use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("In spawn_main_menu");
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu() {

}

pub fn build_main_menu(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    println!("In build_main_menu");
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::BLUE.into(),
            ..default()
        })
        .id()
}
