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
    let rotate_90_x = Rotation::from_rotation_x(1.5708);
    let rotate_90_z = Rotation::from_rotation_z(1.5708);

    // Background
    commands.spawn(
        PbrComponents {
            mesh: asset_server
                .load("assets/background/export/background.gltf")
                .unwrap(),
            material: materials.add(background_color.into()),
            rotation:  rotate_90_x,
            translation: Translation::new(0.0, 0.0, 0.0),
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
            translation: Translation::new(-5.20, 0.0, 3.0),
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
            translation: Translation::new(5.20, 0.0, 3.0),
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
            rotation: rotate_90_z,
            translation: Translation::new(0.22, -10.15, 3.0),
            ..Default::default()
        },
    );

    println!("Arena setup complete");
}

// Test system to setup some tetronimos for debug purposes
pub fn tetronimo_test_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tetronimo_debug_on = true;
    let tetronimo_color = Color::rgb(0.47, 0.16, 0.06);
    let x_min = -4;
    let x_max = 6;
    let y_min = -9;
    let y_max = 11;

    if tetronimo_debug_on {
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
                        translation: Translation::new(x_pos as f32, y_pos as f32, 3.5),
                        ..Default::default()
                    },
                );
            }
        }

        println!("Tetronimo test setup complete");
    }
}
