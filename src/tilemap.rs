use bevy::{
    math::prelude::*,
    prelude::*,
    render::view::{ComputedVisibility, NoFrustumCulling, Visibility},
};
use rand::Rng;

use crate::{
    game::TILE_SIZE,
    mesh_instancing::{CustomMaterialPlugin, InstanceData, InstanceMaterialData},
    GameState,
};

pub const BOARD_X: usize = 50;
pub const BOARD_Y: usize = 50;

pub struct TileMapPlugin;

#[derive(Clone, Copy)]
pub enum TileType {
    Grass,
    Road,
    Water,
}

#[derive(Clone, Copy)]
pub struct Tile<'a> {
    tile_type: TileType,
    instance_data: &'a InstanceData,
}

#[derive(Component)]
pub struct Map<'a> {
    tiles: [[Tile<'a>; BOARD_X]; BOARD_Y],
}

impl Map<'_> {
    fn get_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[x][y]
    }
}

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CustomMaterialPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(create_map));
        // .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_map));
    }
}

fn update_map(mut commands: Commands, mut map_query: Query<&mut InstanceMaterialData>) {
    // if let Ok(mut material_data) = map_query.get_single_mut() {
    //     let mut rng = rand::thread_rng();
    //     material_data.0[rng.gen_range(0..BOARD_X * BOARD_Y)].color =
    //         Color::rgb(rng.gen(), rng.gen(), rng.gen()).as_rgba_f32();
    // }
}

fn create_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let _rand = rand::thread_rng();
    let mut instance_data = Vec::new();
    let green = Color::GREEN;

    let mut data_ref = InstanceData {
        position: Vec3::new(0.0, 0.0, 0.0),
        scale: TILE_SIZE,
        color: green.as_rgba_f32(),
    };

    let grass = Tile {
        tile_type: TileType::Grass,
        instance_data: &data_ref,
    };

    let map = Map {
        tiles: [[grass; BOARD_X]; BOARD_Y],
    };

    for x in 0..BOARD_X {
        for y in 0..BOARD_Y {
            let mut data = *map.tiles[x][y].instance_data;
            data.position = Vec3::new(x as f32 * TILE_SIZE, 0., y as f32 * TILE_SIZE);
            instance_data.push(data);
        }
    }

    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InstanceMaterialData(instance_data),
        Visibility::default(),
        ComputedVisibility::default(),
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));
    // .insert(map);
}
