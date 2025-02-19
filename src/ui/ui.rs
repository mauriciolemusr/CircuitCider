use bevy::{prelude::*, transform::commands};
use super::components::*;

// pub struct MainMenuUI;

// impl Plugin for MainMenuUI {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, spawn_start_menu);
//         //app.add_systems(Update, button_system);
//     }
// }

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenuUI,
    Editor,
}


pub fn despawn_start_menu(
    mut commands: Commands,
    arena_button: Query<Entity, With<StartArenaWidget>>,
    editor_button: Query<Entity, With<StartEditorWidget>>,
    exit_button: Query<Entity, With<ExitAppWidget>>,
    logo: Query<Entity, With<LogoWidget>>,
    title: Query<Entity, With<TitleWidget>>,
    main_menu_background: Query<Entity, With<MainMenuBackgroundWidget>>,

) {
    for button in arena_button.iter() {
        commands.entity(button).despawn_recursive();
    }
    for button in editor_button.iter() {
        commands.entity(button).despawn_recursive();
    }
    for button in exit_button.iter() {
        commands.entity(button).despawn_recursive();
    }
    for widget in logo.iter() {
        commands.entity(widget).despawn_recursive();
    }
    for widget in title.iter() {
        commands.entity(widget).despawn_recursive();
    }
    for widget in main_menu_background.iter() {
        commands.entity(widget).despawn_recursive();
    }
}

pub fn start_arena(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>, With<StartArenaWidget>)>
    ) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                //*color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                println!("starting editor")
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                //*color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                //*color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
pub fn start_editor(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<Button>, With<StartEditorWidget>)>,
    mut app_state: Res<State<AppState>>,
    mut commands: Commands
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                //*color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                println!("Opening the editor");
                commands.insert_resource(NextState(Some(AppState::Editor)));                
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                //*color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                //*color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn exit_app_button(
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    ),
    (Changed<Interaction>, With<Button>, With<ExitAppWidget>)>
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                //*color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;

                std::process::exit(0);
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                //*color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                //*color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn spawn_start_menu(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,

                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),

            ..default()
        },
        Name::new("Main Menu Background"),
        MainMenuBackgroundWidget,

    ));
    commands
        .spawn(
            (
                NodeBundle {
            style: Style {
                max_width: Val::Percent(36.0),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Percent(0.6)),
                left: Val::Px(6.0),
                margin: UiRect::all(Val::Percent(0.45)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        MainMenuBackgroundWidget
            ))
        .with_children(|parent| {
            // text
            parent.spawn((
                TextBundle::from_section(
                    "Circuit Cider",
                    TextStyle {
                        font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                        font_size: 60.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.)),
                    top: Val::Px(-3.0),
                    ..default()
                }),
                Name::new("CircuitCider Text"),
                TitleWidget
            ),
        );
        });

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(200.0),
                margin: UiRect::top(Val::VMin(5.)),
                left: Val::Percent(8.5),
                top: Val::Vw(5.0),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Percent(0.425)),
                ..default()
            },
            // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
            background_color: Color::WHITE.into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        Name::new("Logo"),
        UiImage::new(asset_server.load("bevyteam5_upscaled.png")),
        LogoWidget,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(255.0),
                    height: Val::Px(300.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    left: Val::Percent(7.5),
                    top: Val::Percent(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            Name::new("Button Nodes"),
            MainMenuBackgroundWidget,
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    (
                    ButtonBundle {
                    style: Style {
                        width: Val::Px(241.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        top: Val::Percent(-35.0),
                        left: Val::Percent(64.2),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb_u8(88, 117, 79).into(),
                    ..default()
                    },
                    StartArenaWidget
                )
            )
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "start",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                        .with_style(Style {
                            margin: UiRect{
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                left: Val::Px(10.5),
                                right: Val::Px(10.5),
                            },
                            ..default()
                        }),
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(
                    (
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(241.0),
                            height: Val::Px(75.0),
                            border: UiRect::all(Val::Percent(2.0)),
                            top: Val::Percent(0.0),
                            left: Val::Px(-19.5),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    border_color: Color::BLACK.into(),
                    background_color: Color::rgb_u8(58, 78, 108).into(),
                    ..default()
                    },
                    StartEditorWidget
                )
            )
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "CUSTOMIZE",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect{
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(10.5),
                            right: Val::Px(10.5),
                        },
                        ..default()
                    }),
                ));
            });
    })
        .with_children(|parent| {
            parent
                .spawn(
                    (
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(241.0),
                            height: Val::Px(75.0),
                            border: UiRect::all(Val::Percent(2.0)),
                            top: Val::Percent(35.5),
                            left: Val::Px(-195.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: Color::BLACK.into(),
                        background_color: Color::rgb_u8(148, 52, 52).into(),
                        ..default()
                    },
                    ExitAppWidget,
                )
            )
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                        "EXIT",
                        TextStyle {
                            font: asset_server.load("TauroCondensed-eZrGB.ttf"),
                            font_size: (40.0),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                    .with_style(Style {
                        margin: UiRect{
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(10.5),
                            right: Val::Px(10.5),
                        },
                        ..default()
                    }),
                ));
            });
    });
}
