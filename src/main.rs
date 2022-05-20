use bevy::prelude::*;
use debug::DebugPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

mod debug;
mod game;
mod main_menu;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Loading,
    MainMenu,
    Game,
    Pause,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
}
