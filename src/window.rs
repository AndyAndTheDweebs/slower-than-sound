use bevy::prelude::*;
use bevy::window::{WindowResized};
use crate::ship::*;
use crate::constants::*;

struct InitalWindow {
    width: f32,
    height: f32,
}

/*
* function: setup
*
* type: system startup call
*
* description: called once during inital boot to grab window info and store in InitalWindow struct
*
* parameters:
*   commands: mutable variable used to execute commands
*   windows: mutable variable used for grabbing windows
*   
* return: none
*/

fn setup(mut commands: Commands,mut windows: ResMut<Windows>){
    let mut window = windows.get_primary_mut().unwrap();
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

/*
* function: impl
*
* type: plugin
*
* description: creation of plugin for window, used to be called 
*   
* return: none
*/

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup.system())
        //.add_system(window_resize_event.system())
        //.add_state(AppState::InGame)
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(window_resize_event.system()));
    }
}