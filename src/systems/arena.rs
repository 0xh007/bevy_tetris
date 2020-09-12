use bevy::prelude::*;

// Sets up the game board and walls for the arena
pub fn arena_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Colors
    let background_color = Color::rgb(0.0, 0.00, 0.0);
    let wall_color = Color::rgb(0.0, 0.53, 0.8);
    let rotate_90_x = Rotation::from_rotation_x(1.5708);
    let rotate_90_y_z = Rotation::from_rotation_xyz(0.0, 1.5708, 1.5708);

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
            rotation:  rotate_90_x,
            translation: Translation::new(-25.0, 10.0, 3.0),
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
            rotation: rotate_90_x,
            translation: Translation::new(25.0, 10.0, 3.0),
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
            rotation: rotate_90_y_z,
            translation: Translation::new(0.0, -40.0, 3.0),
            ..Default::default()
        },
    );

    println!("Arena setup complete");
}
