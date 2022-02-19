use bevy::prelude::*;
use pokemon::{battle::BattlePlugin, overworld::OverworldPlugin, AppState};

fn main() {
    // component -> system -> bundle ?
    App::new()
        // boilerplate
        .add_plugins(DefaultPlugins)
        // sets the background color of things
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_state(AppState::Overworld)
        .add_plugin(OverworldPlugin)
        .add_plugin(BattlePlugin)
        // this runs once on the app's startup
        // this is where we init all the bundles
        // .add_startup_system(setup)
        // a system runs every frame
        // .add_system_set(
        //     SystemSet::on_update(AppState::Battle)
        //         // run every some frames
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        // )
        // scoreboard
        // .add_system(app_state_system)
        // exit on escape?
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

// fn app_state_system(app_state: Res<State<AppState>>, mut query: Query<&mut Text>) {
//     let mut text = query.single_mut();
//     text.sections[1].value = format!("{:?}", app_state.current());
// }
