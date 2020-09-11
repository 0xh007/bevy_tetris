mod systems;

use bevy::prelude::*;
use systems::*;

fn main() {
    let mut builder = App::build();

    builder
        .add_startup_system(render_setup.system());

    builder.run();
}
