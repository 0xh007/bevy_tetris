use bevy::prelude::*;

pub fn render_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dComponents {
       transform: Transform::new_sync_disabled(Mat4::face_toward(
           Vec3::new(0.0, 0.0, 0.0),
           Vec3::new(0.0, 0.0, 0.0),
           Vec3::new(0.0, 0.0, 0.0),
       )),
       ..Default::default()
    });

    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        ..Default::default()
    });

    println!("Render setup complete");
}
