use bevy::prelude::*;

use crate::constants::*;

struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        ButtonMaterials {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font: asset_server.load("font/FiraSans-Bold.ttf"),
        }
    }
}

enum MenuButton {
    leftArrow,
    rightArrow,
}
impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::leftArrow => "<".to_string(),
            Self::rightArrow => ">".to_string(),
        }
    }
}
struct button;

struct MenuUI;
fn setup_menu(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "Confirm",
                                    TextStyle {
                                        font: button_materials.font.clone(),
                                        font_size: 20.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        .insert(button);

                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(550.0), Val::Px(250.0)),
                            ..Default::default()
                        },
                        material: materials.add(asset_server.load("ship.png").into()),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "ship stats",
                            TextStyle {
                                font: button_materials.font.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        //width of left vertical fill
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            //left vertical fill
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            material: materials.add(Color::NONE.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                        margin: Rect::all(Val::Auto),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        style: Style {
                                            margin: Rect::all(Val::Px(5.0)),
                                            ..Default::default()
                                        },
                                        text: Text::with_section(
                                            "<",
                                            TextStyle {
                                                font: button_materials.font.clone(),
                                                font_size: 20.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                })
                                .insert(button);
                        });
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                margin: Rect::all(Val::Auto),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    ">",
                                    TextStyle {
                                        font: button_materials.font.clone(),
                                        font_size: 20.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        .insert(button);
                });
        });
}

fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

/*
fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::MakeMap => state
                    .set(AppState::MakeMap)
                    .expect("Couldn't switch state to MakeMap"),
                MenuButton::PlaySong(song) => {
                    let config = load_config(&*format!("{}.toml", song), &asset_server);
                    commands.insert_resource(config);
                    state
                        .set(AppState::Game)
                        .expect("Couldn't switch state to Game")
                }
            };
        }
    }
}
*/

pub struct SelectionMenuPlugin;
impl Plugin for SelectionMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::SelectionMenu).with_system(setup_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::SelectionMenu)
                    .with_system(button_color_system.system()), //.with_system(button_press_system.system()),
            );
        //.add_system_set(SystemSet::on_exit(AppState::SelectionMenu).with_system(despawn_menu.system()));
    }
}
