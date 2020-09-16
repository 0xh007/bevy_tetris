
use bevy::prelude::*;

/// Sets up lighting for the arena
pub fn lighting_setup(
    mut commands: Commands,
) {
    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
        ..Default::default()
    });

    println!("Lighting setup complete");
}
