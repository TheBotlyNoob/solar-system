#![warn(clippy::all)]

use std::f32::consts::FRAC_PI_2;

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    prelude::*,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::{
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
    SelectionEvent,
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
use space::SpaceObject;

mod space;

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
        .add_system(lock_to_object.after(planet_selected).after(planet_orbit))
        .add_system(escape)
        .add_system(escape_event)
        .add_system(fix_glitch);

    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(bevy::time::FixedTimestep::step(1.0))
            .with_system(debug),
    );

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
            Camera3dBundle::default(),
            PickingCameraBundle::default(), // <- Sets the camera to use for picking.
            Fxaa {
                edge_threshold: Sensitivity::High,
                ..default()
            },
        ))
        .insert(MainCamera);

    escape.send(EscapeEvent);

    // planets
    macro_rules! object {
        ($name:ident) => {
            object!($name, StandardMaterial::default(), t, true)
        };

        ($name:ident, $color:expr) => {
            object!(
                $name,
                StandardMaterial {
                    base_color: $color,
                    ..default()
                },
                t,
                false
            )
        };

        ($name:ident, $material:expr, $texture:ident, $has_texture:literal) => {{
            let obj = SpaceObject::$name;
            let mesh = Mesh::from(shape::UVSphere {
                radius: dbg!(obj.scaled_radius()),
                sectors: 64,
                stacks: 64,
            });

            let $texture = if $has_texture {
                Some(asset_server.load(format!("{}.jpg", stringify!($name).to_lowercase())))
            } else {
                None
            };

            let mut obj_id = commands.spawn((
                PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color_texture: $texture.clone(),
                        // not reflective
                        reflectance: 0.0,
                        metallic: 0.0,
                        ..$material
                    }),
                    transform: {
                        let mut t = Transform::from_xyz(dbg!(obj.scaled_distance()), 0.0, 0.0);
                        // flip the planet so it's not sideways
                        t.rotate_x(FRAC_PI_2);
                        t
                    },
                    ..default()
                },
                PickableBundle::default(), // <- Makes the mesh pickable.
            ));
            obj_id.insert(obj);
            obj_id
        }};
    }

    object!(
        Sun,
        StandardMaterial {
            emissive: Color::rgb_linear(255.0, 255.0, 255.0),
            emissive_texture: texture,
            ..default()
        },
        texture,
        true
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

    object!(Mercury);
    object!(Venus);

    object!(Earth);
    object!(EarthMoon);

    object!(Mars);
    object!(Phobos, Color::GRAY);
    object!(Deimos, Color::GRAY);

    object!(Jupiter);
    object!(Io, Color::YELLOW_GREEN);
    object!(Metis, Color::PINK);
    object!(Adrastea, Color::GRAY);
    object!(Amalthea, Color::GRAY);
    object!(Thebe, Color::GRAY);

    object!(Saturn);
    object!(Enceladus, Color::WHITE);
    object!(Mimas, Color::GRAY);
    object!(Tethys, Color::WHITE);
    object!(Dione, Color::WHITE);
    object!(Rhea, Color::BISQUE);
    object!(Titan, Color::ORANGE);

    object!(Uranus);
    object!(Miranda, Color::GRAY);
    object!(Ariel, Color::WHITE);
    object!(Umbriel, Color::GRAY);
    object!(Titania, Color::BLUE);
    object!(Oberon, Color::BLUE);

    object!(Neptune);
    object!(Triton, Color::PINK);
    object!(Nereid, Color::GRAY);
    object!(Proteus, Color::GRAY);
    object!(Larissa, Color::GRAY);
    object!(Halimede, Color::GRAY);

    object!(Pluto);
    object!(Charon, Color::GRAY);
    object!(Nix, Color::GRAY);
    object!(Hydra, Color::GRAY);
    object!(Kerberos, Color::GRAY);
    object!(Styx, Color::GRAY);
}

/// fix the camera glitch where the camera gets flinged into oblivion
/// this is caused by the camera's x and y coordinates being set to absurdly small numbers
fn fix_glitch(mut camera: Query<(&mut Transform, &mut Smoother), With<MainCamera>>) {
    for (mut transform, mut smooth) in camera.iter_mut() {
        if transform.translation.x.abs() < 0.0001 {
            transform.translation.x = 0.0;
            smooth.reset();
        }
        if transform.translation.y.abs() < 0.0001 {
            transform.translation.y = 0.0;
            smooth.reset();
        }
    }
}

/// debug location of the camera
fn debug(camera: Query<&Transform, With<MainCamera>>) {
    for transform in camera.iter() {
        info!("Camera: {:?}", transform.translation);
    }
}

fn planet_orbit(time: Res<Time>, mut planet_q: Query<(&mut Transform, &SpaceObject)>) {
    let mut main_planets = Vec::with_capacity(8);

    for (mut transform, planet) in planet_q
        .iter_mut()
        .filter(|(_, p)| p.orbits() == SpaceObject::Sun)
    {
        transform.translate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(planet.orbital_velocity() * time.delta_seconds()),
        );
        main_planets.push((*transform, *planet));
    }
    for (mut transform, planet, orbit) in planet_q.iter_mut().filter_map(|(t, p)| {
        Some((
            *t,
            p,
            main_planets
                .iter()
                .find_map(move |(_, p2)| if p.orbits() == *p2 { Some(*t) } else { None })?,
        ))
    }) {
        transform.translate_around(
            orbit.translation,
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

fn lock_to_object(
    planet: Query<(&SpaceObject, &Transform), With<CurrentPlanet>>,
    mut movement: EventWriter<MovementEvent>,
) {
    if let Ok((planet, Transform { translation, .. })) = planet.get_single() {
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
        Vec3::new(0.0, 1000.0, 0.0),
        Vec3::ZERO,
        Vec3::Y,
    )));

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
    }
}
