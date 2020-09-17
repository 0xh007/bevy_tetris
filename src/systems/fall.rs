use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;

pub fn fall_system(
    time: Res<Time>,
    mut testris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&Tetronimo, &mut Transform)>,
) {
    for (tetronimo, mut transform) in &mut tetronimo_query.iter() {
        let direction = -1.0;
        let translation = transform.translation_mut();

        let position = Vec3::new(translation.x(), translation.y(), translation.z());
        let cell = testris_grid.update_position(position);

        // Get current grid cell
        // Check cell below
        // Stop movement if at bottom cell
        // Stop movement if cell below is occuppied
        if !TetrisGrid::is_cell_below_occupied(&testris_grid, cell.0, cell.1) {
            *translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
        }
    }
}
