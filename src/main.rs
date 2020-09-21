mod resources;
mod systems;

use bevy::prelude::*;
use bevy::window;
use resources::*;
use systems::*;

fn main() {
    let mut builder = App::build();

    builder
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: 1920,
            height: 1080,
            vsync: true,
            resizable: false,
            mode: window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(TetrisGrid::construct())
        .add_startup_system(render_setup.system())
        .add_startup_system(lighting_setup.system())
        .add_startup_system(arena_setup.system())
        .add_startup_system(tetronimo_test_setup.system())
        .add_system(fall_system.system())
        //.add_system(grid_debug_system.system())
        .add_default_plugins();

    println!("Builder running");
    builder.run();
}
