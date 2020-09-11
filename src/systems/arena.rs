use bevy::prelude::*;

// Sets up the game board and walls for the arena
pub fn arena_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Colors
    let background_color = Color::rgb(0.0, 0.00, 0.0);
    let wall_color = Color::rgb(0.35, 0.85, 1.0);

    // Background
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 100.0,
        })),
        material: materials.add(background_color.into()),
        translation: Translation::new(0.0, 0.0, 0.0),
        rotation: Rotation::from_rotation_x(1.57),
        ..Default::default()
    });

    // Left Wall
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Quad {
            flip: false,
            size: Vec2::new(0.5, 40.0),
        })),
        material: materials.add(wall_color.into()),
        translation: Translation::new(-15.0, 0.0, 1.0),
        ..Default::default()
    });

    // Right Wall
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Quad {
            flip: false,
            size: Vec2::new(0.5, 40.0),
        })),
        material: materials.add(wall_color.into()),
        translation: Translation::new(15.0, 0.0, 1.0),
        ..Default::default()
    });

    // Bottom Wall
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Quad {
            flip: false,
            size: Vec2::new(30.0, 0.5),
        })),
        material: materials.add(wall_color.into()),
        translation: Translation::new(0.0, -19.75, 1.0),
        ..Default::default()
    });

    println!("Arena setup complete");
}
