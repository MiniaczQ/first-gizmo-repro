use std::time::Duration;

use bevy::{
    color::palettes::css::{BLUE, GREEN},
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_gizmo_config(
        DefaultGizmoConfigGroup,
        GizmoConfig {
            render_layers: RenderLayers::layer(1),
            line_width: 40.0,
            ..default()
        },
    );
    app.add_systems(Startup, setup);
    app.add_systems(Update, draw_gizmos);
    app.run();
}

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Texture
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);

    // Cube
    let cube_handle = meshes.add(Cuboid::new(5.0, 5.0, 5.0));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        unlit: true,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle,
        ..default()
    });

    // Gizmo camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                target: image_handle.into(),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(1),
    ));

    // Normal camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 0.0, 0.0).looking_to(Vec3::X, Vec3::Y),
        ..default()
    });
}

fn draw_gizmos(mut gizmos: Gizmos, time: Res<Time>, mut drawn: Local<bool>) {
    if time.elapsed() > Duration::from_secs(3) && !*drawn {
        info!("Drawn");
        gizmos.circle_2d(Vec2::new(0.0, 0.0), 30.0, BLUE);
        gizmos.circle_2d(Vec2::new(0.0, 0.0), 120.0, GREEN);
        *drawn = true;
    }
}
