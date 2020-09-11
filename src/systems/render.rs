use bevy::prelude::*;

pub fn render_setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dComponents {
       transform: Transform::new_sync_disabled(Mat4::face_toward(
           Vec3::new(0.0, 0.0, 50.0),
           Vec3::new(0.0, 0.0, 0.0),
           Vec3::new(0.0, 1.0, 0.0),
       )),
       ..Default::default()
    });

    println!("Render setup complete");
}
