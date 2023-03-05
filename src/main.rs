#![warn(clippy::all)]
#![allow(dead_code)]

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        fxaa::{Fxaa, Sensitivity},
    },
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use smooth_bevy_cameras::{
    controllers::orbit::{
        ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
    },
    LookTransformPlugin,
};

mod planets;

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

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(Msaa { samples: 1 });

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
    .add_plugin(LookTransformPlugin)
    .add_plugin(OrbitCameraPlugin {
        override_input_system: true,
    })
    .add_plugin(bevy_framepace::FramepacePlugin);

    app.add_startup_system(setup);

    app.add_system(orbit_controller);

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

    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true, // 1. HDR must be enabled on the camera
                    ..default()
                },
                ..default()
            },
            BloomSettings {
                scale: 2.5,

                ..default()
            }, // 2. Enable bloom for the camera
            Fxaa {
                edge_threshold: Sensitivity::High,
                ..default()
            },
        ))
        .insert(OrbitCameraBundle::new(
            default(),
            Vec3::new(0.0, 1_000.0, 0.0),
            Vec3::ZERO,
            Vec3::Y,
        ));

    // sun
    let sun_texture = asset_server.load::<Image, _>("planets/sun.jpg");
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 100.0,
            sectors: 64,
            stacks: 64,
        })),
        material: materials.add(StandardMaterial {
            emissive: Color::rgb_linear(250.0, 250.0, 250.0),
            emissive_texture: Some(sun_texture.clone()),
            base_color_texture: Some(sun_texture),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn orbit_controller(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    controllers: Query<&OrbitCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let OrbitCameraController {
        mouse_rotate_sensitivity,
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    if keyboard.pressed(KeyCode::LControl) {
        events.send(ControlEvent::TranslateTarget(
            mouse_rotate_sensitivity * cursor_delta * 100.0,
        ));
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        events.send(ControlEvent::Orbit(
            mouse_translate_sensitivity * cursor_delta,
        ));
    }

    let mut scalar = 1.0;
    for event in mouse_wheel_reader.iter() {
        // scale the event magnitude per pixel or per line
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y / pixels_per_line,
        };
        scalar *= 1.0 - scroll_amount * mouse_wheel_zoom_sensitivity;
    }
    events.send(ControlEvent::Zoom(scalar));
}
