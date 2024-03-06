use bevy::{prelude::*, window::WindowResized};

#[path = "camera/plugin.rs"]
mod camera_plugin;

#[path = "player/plugin.rs"]
mod player_plugin;

#[path = "world/plugin.rs"]
mod world_plugin;

#[path = "entity/plugin.rs"]
mod entity_plugin;

#[derive(Resource)]
pub struct WindowSize(pub Vec2);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .insert_resource(WindowSize(Vec2::new(100.0, 100.0))) // prevents blurry sprites
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (resize_notificator,))
        .add_plugins((
            camera_plugin::CameraPlugin,
            player_plugin::PlayerPlugin,
            world_plugin::WorldPlugin,
            entity_plugin::EntityPlugin,
        ))
        .run();
}

fn resize_notificator(
    mut window_size: ResMut<WindowSize>,
    resize_event: Res<Events<WindowResized>>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.read(&resize_event) {
        window_size.0.x = e.width;
        window_size.0.y = e.height;
    }
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(70.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..8 {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    margin: UiRect {
                                        top: Val::Auto,
                                        bottom: Val::Auto,
                                        left: Val::Px(10.0),
                                        right: Val::Px(10.0),
                                    },
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.50, 0.50, 0.50).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    format!("{i}"),
                                    TextStyle {
                                        font_size: 20.,
                                        ..default()
                                    },
                                ),));
                            });
                    }
                });
        });
}
