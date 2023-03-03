use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: {
                WindowDescriptor {
                    title: "Solar System".to_string(),
                    fit_canvas_to_parent: true,

                    ..default()
                }
            },
            ..default()
        }))
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_startup_system(setup)
        .add_system(move_camera);

    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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
