use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoBlock;
use crate::TetronimoState;

pub fn fall_system(
    time: Res<Time>,
    mut tetris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform, &mut Children)>,
    block_query: Query<(&mut TetronimoBlock, &Transform)>,
) {
    for (mut tetronimo, mut tetronimo_transform, mut children) in &mut tetronimo_query.iter() {
        let mut tetronimo_translation = tetronimo_transform.translation();
        let direction = -1.0;
        let mut collision_detected = false;

        for &child in &mut children.iter() {
            let mut block = block_query.get_mut::<TetronimoBlock>(child).unwrap();
            let block_relative_transform = block_query.get_mut::<Transform>(child).unwrap();
            let block_relative_translation = block_relative_transform.translation();

            let x = tetronimo_translation.x() + block_relative_translation.x();
            let y = tetronimo_translation.y() + block_relative_translation.y();
            let z = tetronimo_translation.z() + block_relative_translation.z();
            let block_translation = Vec3::new(x, y, z);

            let cell = tetris_grid.update_position(block_translation, block.last_grid_pos, block.state);

            if cell != block.current_grid_pos {
                block.last_grid_pos = block.current_grid_pos;
                block.current_grid_pos = cell;
            }
            
            println!("-----");
            println!("{}", block.name);
            println!("{}", block.state);
            println!("Physical Pos ({}, {})", block_translation.x(), block_translation.y());
            println!("-----");

            if TetrisGrid::is_cell_below_occupied(&tetris_grid, cell.0, cell.1) {
                block.state = TetronimoState::Stopped;
                collision_detected = true;
            }
        }

        if !collision_detected {
            *tetronimo_translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
        }
    }
}

// TODO: Remove once new system is fully in place
/*
pub fn fall_system_old(
    time: Res<Time>,
    mut testris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut TetronimoBlock, &GlobalTransform, &mut Transform)>,
) {
    for (mut tetronimo, global_transform, mut transform) in &mut tetronimo_query.iter() {
        let direction = -1.0;
        let global_translation = global_transform.translation();
        let mut translation = transform.translation();

        let position = Vec3::new(global_translation.x(), global_translation.y(), global_translation.z());
        let cell = testris_grid.update_position(position, tetronimo.last_grid_pos, tetronimo.state);

        if cell != tetronimo.current_grid_pos {
            tetronimo.last_grid_pos = tetronimo.current_grid_pos;
            tetronimo.current_grid_pos = cell;
        }

        println!("-----");
        println!("{}", tetronimo.name);
        println!("{}", tetronimo.state);
        println!("Physical Pos ({}, {})", global_translation.x(), global_translation.y());
        println!("Last Grid Pos [{}][{}]", tetronimo.last_grid_pos.0, tetronimo.last_grid_pos.1);
        println!("Current Grid Pos [{}][{}]", tetronimo.current_grid_pos.0, tetronimo.current_grid_pos.1);
        println!("-----");
        
        if !TetrisGrid::is_cell_below_occupied(&testris_grid, cell.0, cell.1) {
            *translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
        } else {
            tetronimo.state = TetronimoState::Stopped;
        }
    }
}
*/

pub fn grid_debug_system(
    mut testris_grid: ResMut<TetrisGrid>,
) {
    TetrisGrid::print_grid(&testris_grid);
}
