use bevy::prelude::*;
mod ship;
use ship::ShipPlugin;
mod window;
use window::WindowPlugin;

fn main() {
    App::build()
    .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins)
    .add_plugin(ShipPlugin)
    .add_plugin(WindowPlugin)
    .run();
}

struct _texture;

struct Player;
struct PlayerXp(u32);
struct PlayerName(String);
struct PlayerHealth(u32);

#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: PlayerHealth,
    _p: Player,

    #[bundle]
    sprite: SpriteBundle,
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
