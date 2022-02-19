use crate::AppState;
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(setup));
        app.add_system_set(SystemSet::on_update(AppState::Battle).with_system(print));
        app.add_system_set(SystemSet::on_exit(AppState::Battle).with_system(x));
        // .add_system_set(
        //     SystemSet::on_update(AppState::Overworld)
        //         // run every some frames
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //         // always take into account paddle movement
        //         .with_system(player_movement_system)
        //         // always take into account ball collision
        //         .with_system(bush_collision_system)
        //         // always take into account ball movement
        //         .with_system(bush_reset_system),
        // )
        // .add_system_set(
        //     SystemSet::on_exit(AppState::Overworld)
        //         // always take into account ball movement
        //         .with_system(exit),
        // )t;
    }
}

fn setup(
    // mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
) {
    // spawn player pokemon
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            // we want player to start at the bottom of the board
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(300.0, 300.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    // TODO insert pokemon struct here later
    // .insert(Player {
    //     velocity: Vec3::default(),
    //     movement_acceleration: 500.,
    //     acceleration: Vec3::default(),
    // });
}

fn print() {
    println!("hello")
}

fn x() {
    println!("exit");
    std::process::exit(0);
}
