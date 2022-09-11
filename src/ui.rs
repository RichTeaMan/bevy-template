use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{camera, config};

#[derive(Component)]
pub struct DebugInfo {
    pub enabled: bool,
}

#[derive(Component)]
pub struct DebugUiInfo;


#[derive(Component)]
pub struct DebugUiResolution;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mono_font = asset_server.load("fonts/FiraMono-Regular.ttf");
    let _regular_font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    #[cfg(debug_assertions)]
    let debug_enabled = true;

    #[cfg(not(debug_assertions))]
    let debug_enabled = false;

    commands.spawn().insert(DebugInfo {
        enabled: debug_enabled,
    });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: mono_font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(DebugUiInfo);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: mono_font,
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..default()
        })
        .insert(DebugUiResolution);
}

pub fn debug_info_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<DebugUiInfo>>,
    debug_query: Query<&DebugInfo>,
) {
    let debug_info = debug_query.single();

    for mut text in query.iter_mut() {
        if debug_info.enabled {
            let mut fps = 0.0;
            if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(fps_avg) = fps_diagnostic.average() {
                    fps = fps_avg;
                }
            }

            text.sections[0].value = format!(
                "Build {}, {} {:.1} fps",
                config::GIT_VERSION,
                config::BUILD_DATE,
                fps
            );
        } else {
            text.sections[0].value = format!("");
        }
    }
}

pub fn update_debug_ui_system(
    windows: Res<Windows>,
    mut query: Query<&mut Text, With<DebugUiResolution>>,
    mut style_query: Query<&mut Style, With<DebugUiResolution>>,
) {
    for mut text in query.iter_mut() {
        let resolution = camera::get_primary_window_size(&windows);

        text.sections[0].value = format!("Resolution:{}, {}", resolution.x, resolution.y);
    }
    for mut text in style_query.iter_mut() {
        text.position = UiRect {
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        };
    }
}
