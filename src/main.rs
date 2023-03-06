#![warn(clippy::all)]

use std::f32::consts::FRAC_PI_2;

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    prelude::*,
};
use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
    SelectionEvent,
};
use planets::Planet;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

mod planets;

#[derive(Component)]
struct CurrentPlanet;

#[derive(Default, Debug, Clone, Copy, PartialEq, Reflect, Resource)]
struct Movement(Transform);

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Movement::default())
        .insert_resource(AmbientLight {
            brightness: 0.5, // represents the brightness of stars around the solar system
            ..Default::default()
        })
        .add_event::<EscapeEvent>();

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
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(OutlinePlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin);

    app.add_startup_system(setup);

    app.add_system(planet_selected)
        .add_system(planet_orbit)
        .add_system(lock_to_planet.after(planet_selected).after(planet_orbit))
        .add_system(escape)
        .add_system(escape_event);

    app.run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut escape: EventWriter<EscapeEvent>,
) {
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(60.0);

    commands
        .spawn(LookTransformBundle {
            transform: LookTransform {
                eye: Vec3::ONE,
                target: Vec3::ZERO,
                up: Vec3::Y,
            },
            smoother: Smoother::new(0.9),
        })
        .insert((
            Camera3dBundle::default(),
            PickingCameraBundle::default(), // <- Sets the camera to use for picking.
            Fxaa {
                edge_threshold: Sensitivity::High,
                ..default()
            },
        ));

    escape.send(EscapeEvent);

    // planets
    macro_rules! planet {
        ($name:ident) => {{
            let planet = Planet::$name;
            let mesh = Mesh::from(shape::UVSphere {
                radius: planet.scaled_radius(),
                sectors: 64,
                stacks: 64,
            });

            let mut planet_id = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(
                            asset_server
                                .load(format!("planets/{}.jpg", stringify!($name).to_lowercase())),
                        ),
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
        }};
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

fn planet_orbit(time: Res<Time>, mut planets: Query<(&mut Transform, &Planet)>) {
    for (mut transform, planet) in planets.iter_mut() {
        transform.translate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(planet.orbital_velocity() * time.delta_seconds()),
        );
    }
}

fn escape(kbd: ResMut<Input<KeyCode>>, mut events: EventWriter<EscapeEvent>) {
    if kbd.just_pressed(KeyCode::Escape) {
        info!("Escape pressed");
        events.send(EscapeEvent);
    }
}

// when a planet is selected, show information about it, zoom in on it, and change the camera's orbit
fn planet_selected(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut cameras: Query<&mut LookTransform, With<Camera3d>>,
    planets: Query<(&Planet, &Transform)>,
    current_planet: Query<Entity, With<CurrentPlanet>>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(entity)) = event {
            if let Ok((planet, transform)) = planets.get(*entity) {
                info!("Selected planet: {:?}", planet);
                if let Ok(planet) = current_planet.get_single() {
                    commands.entity(planet).remove::<CurrentPlanet>();
                }
                commands.entity(*entity).insert(CurrentPlanet);
                let mut camera = cameras.single_mut();
                camera.target = transform.translation;
                camera.eye =
                    transform.translation + Vec3::new(planet.scaled_radius() * 3.0, 6.0, 0.0);
            }
        }
    }
}

fn lock_to_planet(
    planet: Query<(&Planet, &Transform), With<CurrentPlanet>>,
    mut cameras: Query<&mut LookTransform, With<Camera3d>>,
) {
    if let Ok((planet, transform)) = planet.get_single() {
        info!("Locking to planet: {:?}", planet);
        let mut camera = cameras.single_mut();
        camera.target = transform.translation;
        camera.eye = transform.translation + Vec3::new(planet.scaled_radius() * 3.0, 6.0, 0.0);
    }
}

/// An event that makes the camera look at the whole solar system.
struct EscapeEvent;

fn escape_event(
    mut commands: Commands,
    events: EventReader<EscapeEvent>,
    mut cameras: Query<&mut LookTransform, With<Camera3d>>,
    mut planet: Query<Entity, With<CurrentPlanet>>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();
    let mut camera = cameras.single_mut();
    camera.target = Vec3::ZERO;
    camera.eye = Vec3::new(0.0, 1_000_000.0, 0.0);
    info!("Camera reset");
    if let Ok(planet) = planet.get_single_mut() {
        commands.entity(planet).remove::<CurrentPlanet>();
    }
}
