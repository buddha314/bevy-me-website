use bevy::camera::Viewport;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const VIEWPORT_COUNT: u32 = 2;

#[derive(Component)]
struct Spin(pub f32);

#[derive(Component, Clone, Copy)]
struct SceneViewport {
    index: u32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.08)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Multi-Viewport Website".into(),
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (spin_meshes, sync_viewports))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(6.0, 10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let ground_mesh = meshes.add(Cuboid::new(14.0, 0.2, 10.0));
    let ground_material = materials.add(Color::srgb(0.12, 0.14, 0.18));

    commands.spawn((
        Mesh3d(ground_mesh),
        MeshMaterial3d(ground_material),
        Transform::from_xyz(0.0, -0.6, 0.0),
    ));

    spawn_showcase(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-3.0, 0.0, 0.0),
        [
            Color::srgb(0.38, 0.71, 0.95),
            Color::srgb(0.55, 0.86, 0.55),
            Color::srgb(0.98, 0.74, 0.32),
        ],
    );
    spawn_showcase(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(3.0, 0.0, 0.0),
        [
            Color::srgb(0.98, 0.45, 0.61),
            Color::srgb(0.68, 0.56, 0.97),
            Color::srgb(0.45, 0.89, 0.9),
        ],
    );

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.03, 0.04, 0.07)),
            ..default()
        },
        Transform::from_xyz(-3.0, 3.8, 8.5).looking_at(Vec3::new(-3.0, 0.8, 0.0), Vec3::Y),
        SceneViewport { index: 0 },
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.03, 0.04, 0.07)),
            ..default()
        },
        Transform::from_xyz(3.0, 3.8, 8.5).looking_at(Vec3::new(3.0, 0.8, 0.0), Vec3::Y),
        SceneViewport { index: 1 },
    ));
}

fn spawn_showcase(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    center: Vec3,
    colors: [Color; 3],
) {
    for (index, color) in colors.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.35, 1.35, 1.35))),
            MeshMaterial3d(materials.add(color)),
            Transform::from_xyz(center.x + ((index as f32) - 1.0) * 1.7, 0.9, 0.0),
            Spin(0.35 + index as f32 * 0.18),
        ));
    }
}

fn spin_meshes(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut query {
        transform.rotate_y(time.delta_secs() * spin.0);
        transform.rotate_x(time.delta_secs() * spin.0 * 0.5);
    }
}

fn sync_viewports(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<(&SceneViewport, &mut Camera)>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };

    let width = window.physical_width().max(VIEWPORT_COUNT);
    let height = window.physical_height().max(1);
    let viewport_width = width / VIEWPORT_COUNT;

    for (scene_viewport, mut camera) in &mut cameras {
        let left = scene_viewport.index * viewport_width;
        let remaining_width = width.saturating_sub(left);
        let this_width = if scene_viewport.index + 1 == VIEWPORT_COUNT {
            remaining_width
        } else {
            viewport_width
        };

        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(left, 0),
            physical_size: UVec2::new(this_width.max(1), height),
            ..default()
        });
    }
}
