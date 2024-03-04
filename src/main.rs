use bevy::{prelude::*, window::WindowResized};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(WindowSize(Vec2::new(100.0, 100.0))) // prevents blurry sprites
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (animate_sprite, player_movement, resize_notificator),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Stick(Vec3);

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct WindowSize(Vec2);

#[derive(Component)]
struct MovingRight(bool);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut player_query: Query<(&mut Sprite, &MovingRight), With<Player>>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }

    let (mut sprite, moving_right) = player_query.single_mut();

    sprite.flip_x = !moving_right.0;
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &mut MovingRight, &Speed), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    mut stick_query: Query<
        (&mut Transform, &mut Stick),
        (With<Stick>, Without<Player>, Without<Camera2d>),
    >,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor_evr: EventReader<CursorMoved>,
    window_size: Res<WindowSize>,
) {
    let (mut player_transform, mut moving_right, speed) = player_query.single_mut();

    let mut camera_transform = camera_query.single_mut();

    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -1.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
        moving_right.0 = false;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
        moving_right.0 = true;
    }

    player_transform.translation += Vec3::new(
        direction.normalize_or_zero().x,
        direction.normalize_or_zero().y,
        0.0,
    ) * speed.0
        * time.delta_seconds()
        * 100.0;

    camera_transform.translation = player_transform.translation;

    let (mut stick_transform, mut stick) = stick_query.single_mut();

    for ev in cursor_evr.read() {
        let diffx = (window_size.0.x / 2.0) - ev.position.x;
        let diffy = (window_size.0.y / 2.0) - ev.position.y;

        stick.0 = Vec3::new(-diffx, diffy, 0.0).normalize();
    }

    stick_transform.translation = player_transform.translation + stick.0 * 100.0;

    stick_transform.rotation = Quat::from_rotation_arc(Vec3::new(0.0, 1.0, 0.0), stick.0);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/characters/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/characters/stick.png"),
            transform: Transform {
                scale: Vec3::new(3.0, 3.0, 3.0),
                ..default()
            },
            ..default()
        },
        Stick(Vec3::ZERO),
    ));

    let terrain_sprites = [
        "back1.png",
        "back2.png",
        "back3.png",
        "grass1.png",
        "grass2.png",
        "grass3.png",
    ];

    let mut rng = rand::thread_rng();

    for x in -100..100 {
        for y in -100..100 {
            let random_number = rng.gen_range(0..5);

            let randrot = rng.gen::<bool>();

            let mut rotx = if randrot { 0.0 } else { 1.0 };
            let mut roty = if randrot { 1.0 } else { 0.0 };

            if random_number > 2 {
                rotx = 0.0;
                roty = 0.0;
            }

            commands.spawn(SpriteBundle {
                texture: asset_server
                    .load(String::from("textures/terrain/") + terrain_sprites[random_number]),
                transform: Transform {
                    scale: Vec3::new(3.0, 3.0, 3.0),
                    translation: Vec3::new(x as f32 * 32.0 * 3.0, y as f32 * 32.0 * 3.0, -10.0),
                    rotation: Quat::from_xyzw(rotx, roty, 0.0, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Speed(5.0),
        MovingRight(true),
    ));
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
                        parent.spawn(NodeBundle {
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
                        }).with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("{i}"),
                                    TextStyle {
                                        font_size: 20.,
                                        ..default()
                                    },
                                ),
                            ));
                        });
                    }
                });
        });
}
