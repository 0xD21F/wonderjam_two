use bevy::prelude::*;

use crate::GameState;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenuRoot;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy_main_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update));
    }
}

fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenuRoot)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(500.0), Val::Auto),
                            ..default()
                        },
                        image: asset_server.load("ui/logo.png").into(),
                        ..default()
                    });
                });
        });
}

fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(input: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game).unwrap();
    }
}
