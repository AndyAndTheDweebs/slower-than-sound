use bevy::prelude::*;
use bevy::window::{WindowResized};

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_system(window_resize_event.system())
    .run();
}

struct _texture;

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

fn window_resize_event(mut events: EventReader<WindowResized>, mut texture: Query<(&_texture, &mut Transform)>, wnds: Res<Windows>,){
    let wnd = wnds.get_primary().unwrap();
    //loop through all events 
    for event in events.iter() {
        //loop through all entities with component _texture
        for(_texture, mut transform) in texture.iter_mut(){
            //scale all textures with window resize
            //TODO: replace wnd calls as this will currently scale textures incorrectly
            transform.scale = Vec3::new(event.width/wnd.width(), event.height/wnd.height(),0.0);
        }
    }
}
