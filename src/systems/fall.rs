use bevy::prelude::*;
use crate::Tetronimo;

pub fn fall_system(
    time: Res<Time>,
    mut tetronimo_query: Query<(&Tetronimo, &mut Transform)>,
) {
    for (tetronimo, mut transform) in &mut tetronimo_query.iter() {
        let direction = -1.0;
        let translation = transform.translation_mut();
        *translation.y_mut() += time.delta_seconds * direction * tetronimo.speed;
    }
}
