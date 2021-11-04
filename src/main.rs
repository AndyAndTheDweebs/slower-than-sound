use bevy::prelude::*;

fn hello_world_system() {
    println!("Hello World!");
}

fn main() {
    App::build().add_system(hello_world_system.system()).run();
}
