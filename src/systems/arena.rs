use bevy::prelude::*;

// Sets up the game board and walls for the arena
pub fn arena_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Background
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        translation: Translation::new(0.0, 0.0, 0.0),
        rotation: Rotation::from_rotation_x(1.57),
        ..Default::default()
    });

    // Left Wall
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Quad {
            flip: false,
            size: Vec2::new(0.5, 6.0),
        })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        translation: Translation::new(0.0, 0.0, 1.0),
        //rotation: Rotation::from_rotation_x(1.8),
        ..Default::default()
    });

    println!("Arena setup complete");
}
