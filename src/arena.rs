use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const PLANE_SIZE: f32 = 200.0;

pub fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const TILE_SIZE: f32 = 20.0;
    const GROUND_Y_POSITION: f32 = -2.0;

    let ground_plane_handle = meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE }));

    let ground_texture = asset_server.load("ground_texture.png");

    let ground_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(ground_texture),
        perceptual_roughness: 1.0,
        ..default()
    });

    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(PLANE_SIZE, 0.1, PLANE_SIZE))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            GROUND_Y_POSITION,
            0.0,
        )))
        .insert(Friction::coefficient(0.8));

    let tile_count = (PLANE_SIZE / TILE_SIZE) as i32;
    for i in -tile_count..=tile_count {
        for j in -tile_count..=tile_count {
            commands.spawn_bundle(PbrBundle {
                mesh: ground_plane_handle.clone(),
                material: ground_material_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 * TILE_SIZE) as f32,
                    GROUND_Y_POSITION,
                    (j as f32 * TILE_SIZE) as f32,
                )),
                ..default()
            });
        }
    }

    // directional 'sun' light
    const HALF_SIZE: f32 = PLANE_SIZE;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
