#![warn(clippy::all)]
#![allow(dead_code)]

use bevy::{
    core_pipeline::{bloom::BloomSettings, fxaa::Fxaa},
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::CursorGrabMode,
};

enum Planet {
    Sun,
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
}

impl Planet {
    /// The radius of the planet in kilometers.
    fn radius(&self) -> f32 {
        match self {
            Planet::Sun => 695_700.0,
            Planet::Mercury => 2_439.7,
            Planet::Venus => 6_051.8,
            Planet::Earth => 6_371.0,
            Planet::Mars => 3_389.5,
            Planet::Jupiter => 69_911.0,
            Planet::Saturn => 58_232.0,
            Planet::Uranus => 25_362.0,
            Planet::Neptune => 24_622.0,
            Planet::Pluto => 1_188.0,
        }
    }
    /// The average distance from the sun in kilometers.
    fn distance(&self) -> f32 {
        match self {
            Planet::Sun => 0.0,
            Planet::Mercury => 57_909_175.0,
            Planet::Venus => 108_208_930.0,
            Planet::Earth => 149_597_890.0,
            Planet::Mars => 227_936_640.0,
            Planet::Jupiter => 778_412_020.0,
            Planet::Saturn => 1_426_725_400.0,
            Planet::Uranus => 2_870_972_200.0,
            Planet::Neptune => 4_498_252_900.0,
            Planet::Pluto => 5_906_370_000.0,
        }
    }
    /// The speed that the planet orbits the Sun in kilometers/hour.
    /// A positive value means the planet orbits clockwise.
    /// A negative value means the planet orbits counter-clockwise.
    /// A value of 0 means the planet does not orbit.
    /// https://en.wikipedia.org/wiki/Orbital_speed
    fn speed(&self) -> f32 {
        match self {
            Planet::Sun => 0.0,
            Planet::Mercury => 47_872.0,
            Planet::Venus => 35_021.0,
            Planet::Earth => 29_783.0,
            Planet::Mars => 24_077.0,
            Planet::Jupiter => 13_069.0,
            Planet::Saturn => 9_672.0,
            Planet::Uranus => 6_835.0,
            Planet::Neptune => 5_477.0,
            Planet::Pluto => 4_736.0,
        }
    }
    /// The scale of the planet relative to the Sun.
    /// The Sun has a radius of 695,700 km.
    fn scale(&self) -> f32 {
        self.radius() / 6_957.0
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Reflect, Resource)]
struct Movement(Transform);

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Movement::default())
        .insert_resource(AmbientLight {
            brightness: 0.5, // represents the brightness of stars around the solar system
            ..Default::default()
        });

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: {
            WindowDescriptor {
                title: "Solar System".to_string(),
                fit_canvas_to_parent: true,
                cursor_grab_mode: CursorGrabMode::Confined,
                cursor_visible: false,

                ..default()
            }
        },
        ..default()
    }))
    .add_plugin(bevy_framepace::FramepacePlugin);
    // #[cfg(debug_assertions)]
    // app.add_plugin(bevy_editor_pls::EditorPlugin);

    app.add_startup_system(setup);

    app.add_system(mouse_scroll)
        .add_system(mouse_movement)
        .add_system(keyboard_movement)
        .add_system(move_camera.after(mouse_scroll).after(keyboard_movement));

    app.run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },
            transform: Transform::from_xyz(0.0, -500.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 1.0,
            knee: 0.5,
            ..default()
        }, // 2. Enable bloom for the camera
        Fxaa {
            edge_threshold: bevy::core_pipeline::fxaa::Sensitivity::High,
            ..default()
        },
    ));

    // plane for debugging
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    // sun
    let sun_texture = asset_server.load::<Image, _>("planets/sun.jpg");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 100.0,

            ..default()
        })),
        material: materials.add(StandardMaterial {
            emissive: Color::rgb_linear(500.0, 500.0, 500.0),
            emissive_texture: Some(sun_texture.clone()),
            base_color_texture: Some(sun_texture),

            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn mouse_scroll(mut movement: ResMut<Movement>, mut input: EventReader<MouseWheel>) {
    let mut delta = Vec3::ZERO;

    for event in input.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                delta.x -= event.y * 10.0;
            }
            MouseScrollUnit::Pixel => {
                delta.x -= event.y;
            }
        }
    }

    movement.0.translation += delta;
}

fn mouse_movement(mut movement: ResMut<Movement>, mut input: EventReader<MouseMotion>) {
    // make movement look where the mouse is pointing
    let mut delta = Vec2::ZERO;

    for event in input.iter() {
        delta -= event.delta;
    }

    delta /= 4.0;

    movement.0.rotate_x(delta.x.to_radians());
    movement.0.rotate_y(delta.y.to_radians());
}

fn keyboard_movement(mut movement: ResMut<Movement>, keyboard_input: Res<Input<KeyCode>>) {
    let mut delta = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        delta.y += 3.0;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        delta.y -= 3.0;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        delta.z += 3.0;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        delta.z -= 3.0;
    }

    movement.0.translation += delta;
}

fn move_camera(movement: ResMut<Movement>, mut cams: Query<&mut Transform, With<Camera>>) {
    for mut transform in &mut cams {
        *transform = movement.0;
    }
}
