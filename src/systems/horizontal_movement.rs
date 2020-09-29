use bevy::prelude::*;
use crate::TetrisGrid;
use crate::Tetronimo;
use crate::TetronimoBlock;
use crate::TetronimoState;

pub fn horizontal_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut tetris_grid: ResMut<TetrisGrid>,
    mut tetronimo_query: Query<(&mut Tetronimo, &mut Transform)>,
    block_query: Query<(&mut TetronimoBlock, &Transform)>,
) {
    for (mut tetronimo, mut tetronimo_transform) in &mut tetronimo_query.iter() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction -= 1.0;
        }

        let translation = tetronimo_transform.translation_mut();
        let speed = 5.0;
        *translation.x_mut() += time.delta_seconds * direction * speed;
    }
}
