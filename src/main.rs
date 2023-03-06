#![warn(clippy::all)]

use std::f32::consts::FRAC_PI_2;

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    ecs::system::EntityCommands,
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
};
use planets::Planet;
use smooth_bevy_cameras::{
    controllers::orbit::{
        ControlEvent, OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin,
    },
    LookTransform, LookTransformPlugin,
};

mod planets;

#[derive(Default, Debug, Clone, Copy, PartialEq, Reflect, Resource)]
struct Movement(Transform);

fn main() {
    info!("Starting Solar System...");

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
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
    }));

    app.add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin {
            override_input_system: true,
        })
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(OutlinePlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin);

    app.add_startup_system(setup);

    app.add_system(orbit_controller).add_system(planet_selected);

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
            Camera3dBundle::default(),
            PickingCameraBundle::default(), // <- Sets the camera to use for picking.
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

    // planets
    macro_rules! planet {
        ($name:ident) => {
            planet(
                &mut commands,
                &mut meshes,
                &mut materials,
                Planet::$name,
                asset_server.load(format!("planets/{}.jpg", stringify!($name).to_lowercase())),
            )
        };
    }

    planet!(Sun).with_children(|children| {
        children.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::rgb_linear(250.0, 250.0, 250.0),
                intensity: 100_000.0,
                range: 100_000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    });

    planet!(Mercury);
    planet!(Venus);
    planet!(Earth);
    planet!(Mars);
    planet!(Jupiter);
    planet!(Saturn);
    planet!(Uranus);
    planet!(Neptune);
}

fn planet<'w, 's, 'c>(
    commands: &'c mut Commands<'w, 's>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    planet: Planet,
    texture: Handle<Image>,
) -> EntityCommands<'w, 's, 'c> {
    let mesh = Mesh::from(shape::UVSphere {
        radius: planet.scaled_radius(),
        sectors: 64,
        stacks: 64,
    });

    let mut planet_id = commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture),
                // not reflective
                reflectance: 0.0,
                metallic: 0.0,
                ..default()
            }),
            transform: {
                let mut t = Transform::from_xyz(planet.scaled_distance(), 0.0, 0.0);
                // flip the planet so it's not sideways
                t.rotate_x(FRAC_PI_2);
                t
            },
            ..default()
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
    ));
    planet_id
        .insert(OutlineBundle {
            outline: OutlineVolume {
                visible: true,
                colour: Color::WHITE,
                width: 0.5,
            },
            ..default()
        })
        .insert(planet);
    planet_id
}

fn orbit_controller(
    mut events: EventWriter<ControlEvent>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    controllers: Query<&OrbitCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let OrbitCameraController {
        mouse_translate_sensitivity,
        mouse_wheel_zoom_sensitivity,
        pixels_per_line,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
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

// when a planet is selected, show information about it, zoom in on it, and change the camera's orbit
fn planet_selected(
    mut events: EventReader<PickingEvent>,
    mut cameras: Query<&mut LookTransform, With<OrbitCameraController>>,
    planets: Query<(&Planet, &Transform)>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(bevy_mod_picking::SelectionEvent::JustSelected(entity)) =
            event
        {
            if let Ok((planet, transform)) = planets.get(*entity) {
                let mut camera = cameras.single_mut();
                camera.target = transform.translation;
                camera.eye =
                    transform.translation + Vec3::new(planet.scaled_radius() * 3.0, 0.0, 0.0);
            }
        }
    }
}
