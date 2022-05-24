use bevy::prelude::*;
use rand::prelude::*;

use crate::{GameState, ModelAssets};

pub struct GamePlugin;

const BEGIN_GAME_GOLD: i32 = 0;
const BEGIN_GAME_WOOD: i32 = 0;
const BEGIN_GAME_STONE: i32 = 0;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tavern {
    tick_timer: Timer,
    gold_per_tick: i32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GameManager {
    resources: Resources,
}

#[derive(Default, Reflect)]
pub struct Resources {
    gold: i32,
    wood: i32,
    stone: i32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(init_game))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(update_taverns));
    }
}

fn init_game(mut commands: Commands, models: Res<ModelAssets>) {
    let mut rng = rand::thread_rng();

    // Create the game manager
    commands
        .spawn()
        .insert(Name::new("GameManager"))
        .insert(GameManager {
            resources: Resources {
                gold: BEGIN_GAME_GOLD,
                wood: BEGIN_GAME_WOOD,
                stone: BEGIN_GAME_STONE,
            },
        });

    // Create directional light (Sun)
    commands.spawn_bundle(DirectionalLightBundle { ..default() });

    // Create starting tavern
    let tavern_mesh = models.tavern.clone();

    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(rng.gen_range(50.0..200.0), 0., rng.gen_range(50.0..200.0)),
            global: GlobalTransform::identity(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_scene(tavern_mesh);
        })
        .insert(Tavern {
            tick_timer: Timer::from_seconds(1., true),
            gold_per_tick: 5,
        });
}

fn update_taverns(
    mut tavern_query: Query<&mut Tavern>,
    time: Res<Time>,
    mut manager_query: Query<&mut GameManager>,
) {
    let mut game_manager = manager_query.single_mut();

    for mut tavern in tavern_query.iter_mut() {
        tavern.tick_timer.tick(time.delta());
        if tavern.tick_timer.just_finished() {
            game_manager.resources.gold += tavern.gold_per_tick;
        }
    }
}
