use bevy::prelude::*;
use rand::Rng;

pub fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
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
}
