mod arena;
mod camera;
mod models;
mod config;
mod events;
mod input;
mod sounds;
mod ui;

use arena::setup_arena;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier3d::prelude::*;

use events::ControlEvent;
use input::keyboard_input;

use models::spawn_models;
use sounds::setup_sounds;
use ui::{debug_info_system, infotext_system, update_debug_ui_system};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.53, 0.80, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_event::<ControlEvent>()
        .add_startup_system(setup_arena)
        .add_startup_system(setup_sounds)
        .add_startup_system(spawn_models)
        .add_startup_system(camera::spawn_camera)
        .add_startup_system(infotext_system)
        .add_system(camera::pan_orbit_camera)
        .add_system(bevy::window::close_on_esc)
        .add_system(keyboard_input)
        .add_system(update_debug_ui_system)
        .add_system(debug_info_system);

    #[cfg(debug_assertions)]
    {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}
