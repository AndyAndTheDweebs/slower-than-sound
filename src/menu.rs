use bevy::prelude::*;

use crate::constants::*;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

struct MenuUI;

enum MenuButton {
    Play,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Play".to_string();
                *material = button_materials.pressed.clone();
                //app_state.set(AppState::SelectionMenu).unwrap();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Play".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                //text.sections[0].value = "Play".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButton::Play => {
                    state.set(AppState::SelectionMenu).unwrap();
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //buttons.push(MenuButton::Play);
    //buttons.push(MenuButton::Options);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                position: Rect {
                    top: Val::Percent(45.),
                    left: Val::Percent(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(300.0), Val::Px(300.0)),
                    position_type: PositionType::Absolute,
                    //align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                material: materials.add(asset_server.load("woodTexture.png").into()),
                ..Default::default()
            });
        })
        .insert(MenuUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect {
                            top: Val::Percent(10.0),
                            ..Default::default()
                        },
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        //position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Options",
                            TextStyle {
                                font: asset_server.load("font/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuUI);

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect {
                            bottom: Val::Px(50.0),
                            ..Default::default()
                        },
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        //position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(MenuButton::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("font/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuUI);
        });

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(550.0), Val::Px(250.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(20.0),
                    left: Val::Percent(45.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(asset_server.load("pirateShip.png").into()),
            ..Default::default()
        })
        .insert(MenuUI);
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_state(AppState::MainMenu)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(button_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(button_press_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu).with_system(despawn_menu.system()),
            );
    }
}
