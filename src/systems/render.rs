use bevy::prelude::*;

enum CameraType {
    GameCamera,
    SideCamera,
    TraditionalCamera,
}

// Handles camera setup
pub fn render_setup(
    mut commands: Commands,
) {
    let camera_type = CameraType::TraditionalCamera;

    match camera_type { 
        CameraType::GameCamera => {
            println!("Using GameCamera");
            commands.spawn(Camera3dComponents {
               transform: Transform::new_sync_disabled(Mat4::face_toward(
                   Vec3::new(0.0, -18.0, 16.0),
                   Vec3::new(0.0, 2.0, 0.0),
                   Vec3::new(0.0, 1.0, 0.0),
               )),
               ..Default::default()
            });
        },
        CameraType::SideCamera => {
            println!("Using SideCamera");
            commands.spawn(Camera3dComponents {
               transform: Transform::new_sync_disabled(Mat4::face_toward(
                   Vec3::new(-300.0, 0.0, 0.0),
                   Vec3::new(0.0, 0.0, -10.0),
                   Vec3::new(0.0, 1.0, 0.0),
               )),
               ..Default::default()
            });
        },
        CameraType::TraditionalCamera => {
            println!("Using SideCamera");
            commands.spawn(Camera3dComponents {
               transform: Transform::new_sync_disabled(Mat4::face_toward(
                   Vec3::new(0.0, 5.0, 35.0),
                   Vec3::new(0.0, 2.0, 0.0),
                   Vec3::new(0.0, 1.0, 0.0),
               )),
               ..Default::default()
            });
        },
    };

    println!("Render setup complete");
}
