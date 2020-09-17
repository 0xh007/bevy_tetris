use bevy::prelude::*;

// Sets up the game board and walls for the arena
pub fn arena_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Colors
    let background_color = Color::rgb(0.0, 0.001, 0.002);
    let wall_color = Color::rgb(0.08, 0.38, 0.43);
    let rotate_90_x = Quat::from_rotation_x(1.5708);
    let rotate_90_z = Quat::from_rotation_z(1.5708);

    // Background
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/background/export/background.gltf")
                .unwrap(),
            material: materials.add(background_color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(rotate_90_x),
            ..Default::default()
        },
    );

    // Left Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(-5.2, 0.0, 3.0)),
            ..Default::default()
        },
    );

    // Right Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(5.2, 0.0, 3.0)),
            ..Default::default()
        },
    );

    // Bottom Wall
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/bottom_wall/export/bottom_wall.gltf")
                .unwrap(),
            material: materials.add(wall_color.into()),
            transform: Transform::from_translation(Vec3::new(-10.15, -0.22, 3.0))
                .with_rotation(rotate_90_z),
            ..Default::default()
        },
    );

    println!("Arena setup complete");
}

enum TetronimoTest{
    Fill,
    Single,
}

pub struct Tetronimo {
    pub speed: f32,
}

// Test system to setup some tetronimos for debug purposes
pub fn tetronimo_test_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tetronimo_test_type = TetronimoTest::Single;

    let tetronimo_color = Color::rgb(0.47, 0.16, 0.06);

    match tetronimo_test_type {
        TetronimoTest::Fill => {
            let x_min = -4;
            let x_max = 6;
            let y_min = -9;
            let y_max = 11;

            for x in x_min..x_max {
                for y in y_min..y_max {
                    let offset = 0.5;
                    let x_pos = x as f32 - offset;
                    let y_pos = y as f32 - offset;

                    commands.spawn(
                        PbrComponents {
                            mesh: asset_server
                                .load("assets/tetronimo/export/tetronimo.gltf")
                                .unwrap(),
                            material: materials.add(tetronimo_color.into()),
                            transform: Transform::from_translation(Vec3::new(x_pos as f32, y_pos as f32, 3.5)),
                            ..Default::default()
                        },
                    );
                }
            }

            println!("Tetronimo Fill test setup complete");
        },

        TetronimoTest::Single => {
            commands.spawn(
                PbrComponents {
                    mesh: asset_server
                        .load("assets/tetronimo/export/tetronimo.gltf")
                        .unwrap(),
                    material: materials.add(tetronimo_color.into()),
                    transform: Transform::from_translation(Vec3::new(-2.5, -0.5, 3.5)),
                    ..Default::default()
                },
            )
            .with(Tetronimo {
                speed: 2.0,
            });

            println!("Tetronimo Single test setup complete");
        },
    };

}
