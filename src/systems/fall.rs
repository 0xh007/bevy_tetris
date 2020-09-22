use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoState;

pub fn fall_system(
    time: Res<Time>,
    mut testris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform)>,
) {
    for (mut tetronimo, mut transform) in &mut tetronimo_query.iter() {
        let direction = -1.0;
        let translation = transform.translation_mut();

        let position = Vec3::new(translation.x(), translation.y(), translation.z());
        let cell = testris_grid.update_position(position, tetronimo.last_grid_pos, tetronimo.state);

        if cell != tetronimo.current_grid_pos {
            tetronimo.last_grid_pos = tetronimo.current_grid_pos;
            tetronimo.current_grid_pos = cell;
        }

        println!("-----");
        println!("{}", tetronimo.name);
        println!("{}", tetronimo.state);
        println!("Last Pos [{}][{}]", tetronimo.last_grid_pos.0, tetronimo.last_grid_pos.1);
        println!("Current Grid Pos [{}][{}]", tetronimo.current_grid_pos.0, tetronimo.current_grid_pos.1);
        println!("-----");
        
        if !TetrisGrid::is_cell_below_occupied(&testris_grid, cell.0, cell.1) {
            *translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
        } else {
            tetronimo.state = TetronimoState::Stopped;
        }
    }
}

pub fn grid_debug_system(
    mut testris_grid: ResMut<TetrisGrid>,
) {
    TetrisGrid::print_grid(&testris_grid);
}
