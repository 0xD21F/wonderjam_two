use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(init_game));
        // .add_system_set(SystemSet::on_exit(GameState::Game).with_system(destroy_main_menu))
        // .add_system_set(SystemSet::on_update(GameState::Game).with_system(update));
    }
}

fn init_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1 unit = 1 meter
    let ground_mesh = meshes.add(Mesh::from(shape::Plane { size: 100. }));
    let grass_material = materials.add(Color::rgb(0.0, 0.9, 0.0).into());

    let tavern_mesh = meshes.add(Mesh::from(shape::Cube { size: 8. }));
    let tavern_material = materials.add(Color::BEIGE.into());

    commands.spawn_bundle(PbrBundle {
        mesh: ground_mesh,
        // Change material according to position
        material: grass_material,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: tavern_mesh,
        // Change material according to position
        material: tavern_material,
        transform: Transform::from_xyz(0., 4., 0.),
        ..Default::default()
    });

    commands.spawn_bundle(DirectionalLightBundle { ..default() });
}
