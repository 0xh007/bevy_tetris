
use bevy::prelude::*;

/// Sets up lighting for the arena
pub fn lighting_setup(
    mut commands: Commands,
) {
    let x_pos = 0.0;
    let y_pos = 10.0;
    let z_pos = 0.0;

    commands.spawn(LightComponents {
        translation: Translation::new(x_pos, y_pos, z_pos), 
        ..Default::default()
    });

    println!("Lighting setup complete");
}
