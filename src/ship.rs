use crate::constants::*;
use bevy::prelude::*;

struct ShipMaterialResource {
    ship_texture: Handle<ColorMaterial>,
}
//TODO:
//public struct used as mark for window scaling, should be moved to more approriate class
pub struct _texture;

/*
* function: impl
*
* type: initalization
*
* description: used to allow bevy to handle resource initalization once .init_resource is called
*
* return: none
*/

impl FromWorld for ShipMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let ship_handle = asset_server.load("ship.png");

        ShipMaterialResource {
            ship_texture: materials.add(ship_handle.into()),
        }
    }
}

/*
* function: spawn_ship
*
* type: system startup call
*
* description: called once during inital boot to create ship
*
* parameters:
*   commands: mutable variable used to execute commands
*   materials: needed for loading sprites
*
* return: none
*/

fn spawn_ship(mut commands: Commands, materials: Res<ShipMaterialResource>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.ship_texture.clone(),
            sprite: Sprite {
                size: Vec2::new(550.0, 250.0),
                flip_x: true,
                flip_y: false,
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        })
        .insert(_texture);
}

/*
* function: impl
*
* type: plugin
*
* description: creation of plugin for ship, used to be called
*
* return: none
*/

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ShipMaterialResource>()
            //.add_state(AppState::InGame)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_ship.system()));
        //.add_system_set(SystemSet::on_update(AppState::InGame).with_system(test_state.system()));
        //.add_startup_system(spawn_ship.system());
    }
}
