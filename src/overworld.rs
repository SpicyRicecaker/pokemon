use crate::{
    despawn_screen, ActiveState, AppState, Collider, OnOverworldScreen, Player, TIME_STEP,
};

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::random;
pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Overworld)
                // run every some frames
                // THIS OVERRIDES STATE AS SHOWN ABOVE!!!!!!
                // .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // always take into account paddle movement
                .with_system(player_movement_system)
                // always take into account ball movement
                .with_system(bush_reset_system)
                // always take into account ball collision
                .with_system(bush_collision_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::Overworld).with_system(setup))
        .add_system_set(
            SystemSet::on_exit(AppState::Overworld)
                // always take into account ball movement
                .with_system(despawn_screen::<OnOverworldScreen>),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world
    // the board is around 800 x 600, with 0,0 being the center of the board
    // spawn_bundle spawns an entity (that can be queried)
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                // we want player to start at the bottom of the board
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            velocity: Vec3::default(),
            movement_acceleration: 500.,
            // acceleration: Vec3::default(),
        })
        .insert(OnOverworldScreen);
    // ball
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         transform: Transform {
    //             scale: Vec3::new(30.0, 30.0, 0.0),
    //             translation: Vec3::new(0.0, -50.0, 1.0),
    //             ..Default::default()
    //         },
    //         sprite: Sprite {
    //             color: Color::rgb(1.0, 0.5, 0.5),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Ball {
    //         velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
    //     });
    // scoreboard
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/CascadiaCode.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/CascadiaCode.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnOverworldScreen);

    // Add walls
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let wall_thickness = 10.0;
    // bounds are 900 x 600, what if I increase this?
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(OnOverworldScreen);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(OnOverworldScreen);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(OnOverworldScreen);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(OnOverworldScreen);

    // Add bricks
    let brick_rows = 5;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec3::new(30.0, 30.0, 1.0);
    // the full width of the brick grid (not including the margin on the left and right)
    // let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;

    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(
        -bounds.x / 2.0 + 2. * brick_spacing,
        3. * brick_spacing,
        0.0,
    );
    let brick_color = Color::rgb(0.5, 0.5, 1.0);
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            // brick
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: brick_color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: brick_position,
                        scale: brick_size,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Collider::Passthrough)
                .insert(ActiveState::Inactive)
                .insert(OnOverworldScreen);
        }
    }
}
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();
    let mut movement_acceleration = Vec3::default();
    if keyboard_input.pressed(KeyCode::Left) {
        movement_acceleration.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        movement_acceleration.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        movement_acceleration.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        movement_acceleration.y += 1.0;
    }
    // acceleration can be uncapped, but velocity must be

    let translation = &mut transform.translation;
    // move the player horizontally
    // move the player vertically
    player.velocity = player.movement_acceleration * movement_acceleration;
    *translation += (player.velocity * TIME_STEP)
        // bound the player within the walls
        .min(Vec3::new(380.0, 380.0, 0.0))
        .max(Vec3::new(-380.0, -380.0, -0.0));
}

fn bush_reset_system(
    player_query: Query<(&Player, &Transform)>,
    mut bush_query: Query<(&mut Sprite, &Transform, &mut ActiveState)>,
) {
    let (_, player_transform) = player_query.single();
    let ball_size = player_transform.scale.truncate();

    // the query actually gives a list of all entities and colliders
    for (mut sprite, transform, mut active_state) in bush_query.iter_mut() {
        let collision = collide(
            player_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_none() {
            // set the color back to normal
            sprite.color = Color::rgb(0.5, 0.5, 1.0);
            *active_state = ActiveState::Inactive;
        }
    }
}

fn bush_collision_system(
    // commands: Commands,
    // scoreboard: ResMut<Scoreboard>,
    // mut ball_query: Query<(&mut Ball, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
    // query all values that are an entity, and have a transform and sprite quality
    mut collider_query: Query<(Entity, &Transform, &mut Sprite, &mut ActiveState)>,
    // app state for when we need to transition to battle
    mut app_state: ResMut<State<AppState>>,
) {
    let (_, player_transform) = player_query.single();
    let ball_size = player_transform.scale.truncate();

    // the query actually gives a list of all entities and colliders
    for (_, transform, mut sprite, mut active_state) in collider_query.iter_mut() {
        let collision = collide(
            player_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_some() && *active_state == ActiveState::Inactive {
            sprite.color = Color::rgb(0.0, 0.5, 1.0);
            *active_state = ActiveState::Active;

            // set our state
            if random::<f32>() > 0.9 {
                app_state.set(AppState::Battle).unwrap();
                return;
            }
        }
    }
}
