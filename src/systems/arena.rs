use bevy::prelude::*;

pub fn render_arena_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        ..Default::default()
    });

    println!("Arena setup complete");
}
