use bevy::prelude::*;
use crate::TetrisGrid;
use crate::TetronimoBlock;
use crate::TetronimoState;

pub fn horizontal_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut tetris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform, &mut Children)>,
    block_query: Query<(&mut TetronimoBlock, &Transform)>,
) {
}
