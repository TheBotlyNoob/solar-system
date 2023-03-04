use bevy::{
    core_pipeline::{bloom::BloomSettings, fxaa},
    prelude::*,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: {
            WindowDescriptor {
                title: "Solar System".to_string(),
                fit_canvas_to_parent: true,

                ..default()
            }
        },
        ..default()
    }))
    .add_plugin(bevy_framepace::FramepacePlugin);

    app.add_startup_system(setup);

    app.add_system(move_camera);

    #[cfg(debug_assertions)]
    app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin);

    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);

    // sphere light
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                sectors: 128,
                stacks: 64,
                radius: 1.0,

                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                emissive: Color::YELLOW,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0.6, 0.0).with_scale(Vec3::splat(0.1)),
            ..default()
        })
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    radius: 0.1,
                    color: Color::rgb(0.2, 0.2, 1.0),
                    ..default()
                },
                ..default()
            });
        });

    // camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-1.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // post processing
    commands.spawn((
        BloomSettings {
            intensity: 100.0,

            ..default()
        },
        fxaa::Fxaa {
            enabled: true,
            edge_threshold: fxaa::Sensitivity::High,
            ..default()
        },
    ));
}

fn move_camera(input: Res<Input<KeyCode>>, mut cams: Query<&mut Transform, With<Camera3d>>) {
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);

    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        z -= 0.5;
    }
    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        z += 0.5;
    }
    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        x += 0.5;
    }
    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Right) {
        x -= 0.5;
    };

    if input.pressed(KeyCode::LShift) {
        y += 0.5;
    } else if input.pressed(KeyCode::LControl) {
        y -= 0.5;
    }

    let mv = Vec3::new(x, y, z);

    for mut t in &mut cams {
        t.translation += mv;
    }
}
