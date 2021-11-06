use bevy::prelude::*;
mod ship;
use ship::ShipPlugin;
mod window;
use window::WindowPlugin;
mod constants;
use constants::*;
mod menu;
use menu::MenuPlugin;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(ShipPlugin)
    //.add_plugin(WindowPlugin)
    .add_plugin(MenuPlugin)
    .add_startup_system(setup.system())
    //.add_state(AppState::MainMenu)
    //.add_system_set(
    //    SystemSet::on_update(AppState::MainMenu)
    //    .with_system(helloWorld.system())
    //)
    .run();
}
fn helloWorld(){
    println!("hello world");
}

/*
* function: setup
*
* type: system startup call
*
* description: called once during inital boot to create camera
*
* parameters:
*   commands: mutable variable used to execute commands
*   
* return: none
*/

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
