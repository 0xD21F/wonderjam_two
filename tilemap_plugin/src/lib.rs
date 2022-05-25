pub struct TilemapPlugin;
pub mod components;
pub mod mesh_instancing;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use resources::tile_map::TileMap;
use resources::TileMapOptions;

use crate::components::Coordinates;
use crate::mesh_instancing::*;
use crate::resources::tile::TileType;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Loaded tile map plugin");
    }
}

impl TilemapPlugin {
    /// System to generate the complete map
    pub fn create_map(
        mut commands: Commands,
        map_options: Option<Res<TileMapOptions>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let options = match map_options {
            None => TileMapOptions::default(), // If no options is set we use the default ones
            Some(options) => options.clone(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.create_road();
        log::info!("{}", tile_map.console_output());

        // Spawn tiles and instance data for mesh instancing (maybe generate a mesh instead??? which is easier to work with at runtime? does it matter?)
        let grass_colour = Color::rgb(0.0, 0.9, 0.0);
        let road_colour = Color::rgb(0.8, 0.4, 0.0);

        let mut instance_data = Vec::new();

        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let data = InstanceData {
                    position: Vec3::new(y as f32, 0.0, x as f32),
                    scale: 1.0,
                    color: if tile.tile_type == TileType::Grass {
                        grass_colour.as_rgba_f32()
                    } else if tile.tile_type == TileType::Road {
                        road_colour.as_rgba_f32()
                    } else {
                        grass_colour.as_rgba_f32()
                    },
                };
                instance_data.push(data);

                // .spawn_bundle(PbrBundle {
                //     mesh: ground_mesh.clone(),
                //     material: if tile.tile_type == TileType::Grass {
                //         grass_material.clone()
                //     } else if tile.tile_type == TileType::Road {
                //         road_material.clone()
                //     } else {
                //         grass_material.clone()
                //     },
                //     transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                //     ..default()
                // })
                // .insert(Name::new(format!("Tile ({}, {})", x, y)))
                // // We add the `Coordinates` component to our tile entity
                // .insert(Coordinates {
                //     x: x as u16,
                //     y: y as u16,
                // });
            }
        }

        commands
            .spawn()
            .insert_bundle((
                meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
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
            ))
            .insert(Name::new("Tile"));

        // commands
        //     .spawn()
        //     .insert(Name::new("Map"))
        //     .insert(Transform::identity())
        //     .insert(GlobalTransform::default())
        //     .with_children(|parent| {
        //         // Tiles
        //     });
    }
}
