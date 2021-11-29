use bevy::{prelude::*, render::camera::Camera};
mod ship;
use ship::ShipPlugin;
mod window;
use window::WindowPlugin;
mod constants;
use constants::*;
mod menu;
use menu::MenuPlugin;
mod selectionMenu;
use bevy_prototype_parallax::{Layer, LayerBundle, ParallaxPlugin, WindowSize};
use selectionMenu::SelectionMenuPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Slower Than Sound".to_string(),
            resizable: false,
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShipPlugin)
        .add_plugin(WindowPlugin)
        .add_plugin(SelectionMenuPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(ParallaxPlugin)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(follow_player_camera.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(despawn_parallax.system())
                .with_system(reset_camera.system()),
        )
        //.add_state(AppState::MainMenu)
        //)
        .run();
}

fn follow_player_camera(mut camera: Query<&mut Transform, (With<Camera>, Without<UICamera>)>) {
    for mut transform in camera.iter_mut() {
        transform.translation.x += 1.0;
    }
}

fn reset_camera(mut camera: Query<&mut Transform, (With<Camera>, Without<UICamera>)>) {
    for mut transform in camera.iter_mut() {
        transform.translation.x = 0.0;
    }
}

struct MainCamera;
struct UICamera;

/*
* function: despawn_parallax
*
* type: system exit AppState::MainMenu
*
* description: called once the user leaves the MainMenu state, used to remove the parallax background
*
* parameters:
*   commands: mutable variable used to execute commands
*   query: query lookup to find bundle attached to camera responsible for parallax
*
* return: none
*/

fn despawn_parallax(mut commands: Commands, query: Query<Entity, With<MainCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut layer = |path: &str, speed: f32| -> LayerBundle {
        let handle = {
            let handle = asset_server.load(path);
            let color = materials.add(handle.into());
            color
        };
        LayerBundle {
            layer: Layer {
                speed: speed,
                ..Default::default()
            },
            material: handle,
            transform: Transform {
                scale: Vec3::new(0.35, 2.25, 1.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }
    };
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(WindowSize::default())
        .with_children(|f| {
            f.spawn_bundle(layer("ocean.png", 0.1))
                .insert(MainCamera)
                .id();
        });
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UICamera);
}
