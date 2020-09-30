use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoBlock;
use crate::TetronimoState;
use crate::TetronimoDirection;

pub fn horizontal_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform)>,
) {
    for (mut tetronimo, mut tetronimo_transform) in &mut tetronimo_query.iter() {
        let mut direction = 0.0;
        let mut tetronimo_translation = tetronimo_transform.translation_mut();

        if keyboard_input.pressed(KeyCode::Left) && !tetronimo.traveling {
            // Approximate the current position on the x-axis
            let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
            let mut x_destination = (trans_x_rnd - 1.0) as f32;

            // Bound the tetronimo within the arena
            if x_destination > -4.5 {
                tetronimo.traveling = true;
                tetronimo.destination = Vec3::new(x_destination, 0.0, 0.0);
                tetronimo.movement_direction = TetronimoDirection::Left;
            }
        }

        if keyboard_input.pressed(KeyCode::Right) && !tetronimo.traveling {
            // Approximate the current position on the x-axis
            let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
            let mut x_destination = (trans_x_rnd + 1.0) as f32;
            
            // Bound the tetronimo within the arena
            if x_destination < 5.5 {
                tetronimo.traveling = true;
                tetronimo.destination = Vec3::new(x_destination, 0.0, 0.0);
                tetronimo.movement_direction = TetronimoDirection::Right;
            }
        }

        if tetronimo.traveling {
            // Approximate the current position on the x-axis
            let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
            if trans_x_rnd == tetronimo.destination.x() {
                tetronimo.traveling = false;
                tetronimo.movement_direction = TetronimoDirection::None;
            } else {
                let mut direction = 0.0; 
                match tetronimo.movement_direction {
                    TetronimoDirection::None => {
                    },
                    TetronimoDirection::Left => { 
                        direction -= 1.0;
                    }
                    TetronimoDirection::Right => {
                        direction += 1.0;
                    }
                }
                *tetronimo_translation.x_mut() += time.delta_seconds * direction * tetronimo.lateral_speed;
                
                // bound the tetronimo within the arena
                //*tetronimo_translation.x_mut() = tetronimo_translation.x().min(-4.5).max(5.5);
            }
        }
    }
}
