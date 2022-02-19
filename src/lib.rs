use bevy::prelude::*;
pub mod battle;
pub mod overworld;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Overworld,
    Battle,
}

/// An implementation of the classic game "Breakout"
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct Player {
    movement_acceleration: f32,
    // acceleration: Vec3,
    velocity: Vec3,
}

#[derive(Component, PartialEq, Eq)]
/// objects with active states will be queried and made to return to normal
enum ActiveState {
    Active,
    Inactive,
}

#[derive(Component)]
// useful to distinguish different types of collision objects.
// for example, you wouldn't want bushes to be hard stops
enum Collider {
    Solid,
    Passthrough,
}

// struct Scoreboard {
//     score: usize,
// }

#[derive(Component)]
// used to tag entities in the overworld, so we can easily query for them and
// despawn them when needed. See bevy examples, `game_menu.rs`
struct OnOverworldScreen;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
