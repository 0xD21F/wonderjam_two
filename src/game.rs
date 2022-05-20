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
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 100. }));
    let grass_material = materials.add(Color::rgb(0.0, 0.9, 0.0).into());

    commands.spawn_bundle(PbrBundle {
        mesh: mesh,
        // Change material according to position
        material: grass_material,
        ..Default::default()
    });
}
