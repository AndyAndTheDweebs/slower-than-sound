use bevy::prelude::*;
use bevy::window::{WindowResized};

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(add_ship.system())
    .add_system(window_resize_event.system())
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

struct InitalWindow {
    width: f32,
    height: f32,
}

/*
* function: add_ship
*
* type: system startup call
*
* description: called once during inital boot to create ship
*
* parameters:
*   commands: mutable variable used to execute commands
*   asset_server: needed for loading sprites
*   materials: needed for loading sprites
*   
* return: none
*/

fn add_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.
        spawn_bundle(SpriteBundle {
            material:materials.add(asset_server.load("ship.png").into()),
            sprite: Sprite{ size: Vec2::new(550.0, 250.0), flip_x: true, flip_y: false, resize_mode: SpriteResizeMode::Manual },
            ..Default::default()
        })
        .insert(_texture);
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
    mut commands: Commands,mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
    commands.insert_resource(InitalWindow {
        width: window.width(),
        height: window.height(),
    });
}

/*
* function: window_resize_event
*
* type: system call
*
* description: takes a call for a window resize and scales all textures to that size
*
* parameters:
*   events: mutable variable used for storing window resized events
*   texture: mutable variable used for storing every component with a sprite/texture found from query
*   
* return: none
*/

fn window_resize_event(mut events: EventReader<WindowResized>, mut texture: Query<(&_texture, &mut Transform)>, window_size: Res<InitalWindow>){
    //loop through all events 
    for event in events.iter() {
        //loop through all entities with component _texture
        for(_texture, mut transform) in texture.iter_mut(){
            //scale all textures with window resize
                transform.scale = Vec3::new(event.width/(window_size.width), event.height/(window_size.height),0.0);
        }
    }
}

