use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_position = Transform::from_translation(Vec3::new(0.0, 4.1, 0.0))
        .with_rotation(Quat::from_rotation_y(45.0_f32.to_radians()));

    create_model(&mut commands, &asset_server, spawn_position);
}

fn create_model<'w, 's>(
    commands: &mut Commands<'w, 's>,
    asset_server: &Res<AssetServer>,
    spawn_transform: Transform,
) {
    let body_gltf: Handle<Scene> = asset_server.load("basic.glb#Scene0");

    let mut body_commands = commands.spawn();

    body_commands
        .insert_bundle(SpatialBundle::from(spawn_transform))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(2.0, 2.0, 2.0))
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: body_gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(0.0_f32.to_radians()))
                    .with_scale(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            });
        });
}
