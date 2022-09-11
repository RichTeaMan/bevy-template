use bevy::prelude::*;

pub fn setup_sounds(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let background: Handle<AudioSource> = asset_server.load("sounds/ambience.ogg");

    audio.play_with_settings(
        background,
        PlaybackSettings {
            repeat: true,
            volume: 0.4,
            ..Default::default()
        },
    );
}
