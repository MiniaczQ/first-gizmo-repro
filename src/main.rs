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
            line_joints: GizmoLineJoint::Round(16),
            ..default()
        },
    );
    app.add_systems(Startup, setup);
    app.add_systems(Update, draw_first_gizmo.pipe(draw_second_gizmo));
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

/// For some reason, we need to call gizmo draws for N frames before they actually get drawn.
///
/// NOTE: This varies between launches and probably machines too.
///       Adjust this until the first gizmo changes between being drawn and not being drawn in different executions.
///       Then it's only a matter of time before you see the segment glitch.
const DRAW_FOR_N_FRAMES: u64 = 4;

fn draw_first_gizmo(mut gizmos: Gizmos, mut counter: Local<u64>) -> bool {
    if *counter < DRAW_FOR_N_FRAMES {
        // 4 things can happen:
        // - gizmo is not drawn at all - if happens in every execution, increase frames
        // - gizmo is always drawn correctly - if happens in every execution, decrease frames
        // - gizmo is missing joings - happens occasionally when frames are just right
        // - gizmo has joints but no line segments - only happened once for me
        gizmos.circle_2d(Vec2::new(0.0, 0.0), 100.0, BLUE);
        *counter += 1;
        false
    } else {
        true
    }
}

fn draw_second_gizmo(first_draw_done: In<bool>, mut gizmos: Gizmos) {
    if *first_draw_done {
        // Second draw doesn't matter, it's just a reference to how the circle should look like.
        gizmos.circle_2d(Vec2::new(0.0, 0.0), 200.0, GREEN);
    }
}
