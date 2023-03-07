#![warn(clippy::all)]

use std::f32::consts::FRAC_PI_2;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        fxaa::{Fxaa, Sensitivity},
    },
    prelude::*,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::{
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
    SelectionEvent,
};
use planets::Planet;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

mod planets;

#[derive(Component)]
struct CurrentPlanet;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            brightness: 0.5, // represents the brightness of stars around the solar system
            ..Default::default()
        })
        .add_event::<MovementEvent>()
        .add_event::<EscapeEvent>();

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(Msaa { samples: 1 });

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: {
                    WindowDescriptor {
                        title: "Solar System".to_string(),
                        fit_canvas_to_parent: true,

                        ..default()
                    }
                },
                ..default()
            })
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    );

    app.add_plugin(LookTransformPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin);

    app.add_startup_system(setup);

    app.add_system(movement_event)
        .add_system(planet_selected)
        .add_system(planet_orbit)
        .add_system(lock_to_planet.after(planet_selected).after(planet_orbit))
        .add_system(escape)
        .add_system(escape_event);

    #[cfg(debug_assertions)]
    app.add_plugin(bevy_editor_pls::prelude::EditorPlugin);

    app.run()
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut escape: EventWriter<EscapeEvent>,
) {
    commands
        .spawn(LookTransformBundle {
            smoother: Smoother::new(0.8),
            transform: LookTransform::new(Vec3::ONE, Vec3::ZERO, Vec3::Y),
        })
        .insert((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 1_000_000.0, 0.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            #[cfg(not(target_arch = "wasm32"))]
            BloomSettings::default(),
            PickingCameraBundle::default(), // <- Sets the camera to use for picking.
            Fxaa {
                edge_threshold: Sensitivity::High,
                ..default()
            },
        ))
        .insert(MainCamera);

    escape.send(EscapeEvent);

    // planets
    macro_rules! planet {
        ($name:ident) => {
            planet!($name, StandardMaterial::default(), t)
        };
        ($name:ident, $material:expr, $texture:ident) => {{
            let planet = Planet::$name;
            let mesh = Mesh::from(shape::UVSphere {
                radius: planet.scaled_radius(),
                sectors: 64,
                stacks: 64,
            });

            let $texture =
                asset_server.load(format!("planets/{}.jpg", stringify!($name).to_lowercase()));

            let mut planet_id = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some($texture.clone()),
                        // not reflective
                        reflectance: 0.0,
                        metallic: 0.0,
                        ..$material
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
            planet_id.insert(planet);
            planet_id
        }};
    }

    planet!(
        Sun,
        StandardMaterial {
            emissive: Color::rgb_linear(255.0, 255.0, 255.0),
            emissive_texture: Some(texture),
            ..default()
        },
        texture
    )
    .with_children(|children| {
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
    current_planet: Query<Entity, With<CurrentPlanet>>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(entity)) = event {
            info!(?entity, "Selected planet");
            if let Ok(planet) = current_planet.get_single() {
                commands.entity(planet).remove::<CurrentPlanet>();
            }
            commands.entity(*entity).insert(CurrentPlanet);
        }
    }
}

fn lock_to_planet(
    planet: Query<(&Planet, &Transform), With<CurrentPlanet>>,
    mut movement: EventWriter<MovementEvent>,
) {
    if let Ok((planet, Transform { translation, .. })) = planet.get_single() {
        info!(?planet, "Locking to planet");

        movement.send(MovementEvent(LookTransform::new(
            *translation - Vec3::new(0.0, 0.0, planet.scaled_radius() * 3.0),
            *translation,
            Vec3::Y,
        )));
    }
}

/// An event that makes the camera look at the whole solar system.
struct EscapeEvent;

fn escape_event(
    mut commands: Commands,
    events: EventReader<EscapeEvent>,
    mut movement: EventWriter<MovementEvent>,
    mut planet: Query<Entity, With<CurrentPlanet>>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    movement.send(MovementEvent(LookTransform::new(
        Vec3::new(0.0, 100_000.0, 0.0),
        Vec3::ZERO,
        Vec3::Y,
    )));

    info!("Camera reset");
    if let Ok(planet) = planet.get_single_mut() {
        commands.entity(planet).remove::<CurrentPlanet>();
    }
}

struct MovementEvent(LookTransform);

fn movement_event(
    mut events: EventReader<MovementEvent>,
    mut cameras: Query<&mut LookTransform, With<MainCamera>>,
) {
    if let Some(event) = events.iter().last() {
        let mut camera = cameras.single_mut();

        *camera = event.0;

        info!(translate = ?event.0, "Camera moved");
    }
}
