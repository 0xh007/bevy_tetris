use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoBlock;
use crate::TetronimoState;
use crate::TetronimoDirection;

pub fn horizontal_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut tetris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform, &mut Children)>,
    block_query: Query<(&mut TetronimoBlock, &Transform)>,
) {
    for (mut tetronimo, mut tetronimo_transform, mut children) in &mut tetronimo_query.iter() {
        let tetronimo_translation = tetronimo_transform.translation_mut();

        // Ask all child blocks if we can move left/right
        let mut can_move_left = true;
        let mut can_move_right = true;

        for &child in &mut children.iter() {
            let mut block = block_query.get_mut::<TetronimoBlock>(child).unwrap();
            let block_relative_transform = block_query.get_mut::<Transform>(child).unwrap();
            let mut block_relative_translation = block_relative_transform.translation();

            let x = tetronimo_translation.x() + block_relative_translation.x();
            let y = tetronimo_translation.y() + block_relative_translation.y();
            let z = tetronimo_translation.z() + block_relative_translation.z();
            let block_translation = Vec3::new(x, y, z);

            // Get current cell of block
            let cell = tetris_grid.get_cell_from_pos(block_translation);

            // Be sure not to overwrite the flags once they're set to false
            if can_move_left {
                can_move_left = !tetris_grid.is_cell_left_occupied(cell.0, cell.1);
            }
            if can_move_right {
                can_move_right = !tetris_grid.is_cell_right_occupied(cell.0, cell.1);
            }
        }

        if keyboard_input.pressed(KeyCode::Left) && !tetronimo.traveling_laterally {
            // Approximate the current position on the x-axis
            if can_move_left {
                let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
                let x_destination = (trans_x_rnd - 1.0) as f32;

                tetronimo.traveling_laterally = true;
                tetronimo.destination = Vec3::new(x_destination, 0.0, 0.0);
                tetronimo.movement_direction = TetronimoDirection::Left;
            }
        }

        if keyboard_input.pressed(KeyCode::Right) && !tetronimo.traveling_laterally {
            // Approximate the current position on the x-axis
            if can_move_right {
                let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
                let x_destination = (trans_x_rnd + 1.0) as f32;
                
                tetronimo.traveling_laterally = true;
                tetronimo.destination = Vec3::new(x_destination, 0.0, 0.0);
                tetronimo.movement_direction = TetronimoDirection::Right;
            }
        }

        if tetronimo.traveling_laterally {
            // Approximate the current position on the x-axis
            let trans_x_rnd = (tetronimo_translation.x() * 100.0).round() / 100.0;
            if trans_x_rnd == tetronimo.destination.x() {
                tetronimo.traveling_laterally = false;
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
            }
        }
    }
}
