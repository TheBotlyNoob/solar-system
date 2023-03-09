#![warn(clippy::all)]

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    prelude::*,
};
use bevy_dolly::{dolly::glam, prelude::*};
use bevy_egui::{
    egui::{self, RichText},
    EguiContext, EguiPlugin,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_mod_picking::{
    InteractablePickingPlugin, PickableBundle, PickingCameraBundle, PickingEvent, PickingPlugin,
    SelectionEvent,
};
use space::SpaceObject;

mod space;

const DEFAULT_CAMERA_POSITION: glam::Vec3 = glam::Vec3::new(0.0, 100.0, 100_000.0);

#[derive(Component)]
struct CurrentObject;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            brightness: 0.5, // represents the brightness of stars around the solar system
            ..Default::default()
        });

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

    app.add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(FramepacePlugin)
        .add_plugin(EguiPlugin);

    app.add_dolly_component(MainCamera);

    app.add_startup_system(setup);

    app.add_system(object_selected)
        .add_system(planet_orbit)
        .add_system(lock_to_object.after(object_selected).after(planet_orbit))
        .add_system(escape.after(object_selected))
        .add_system(reset_camera.after(escape).after(lock_to_object));

    app.add_system(main_ui).add_system(obj_info_ui);

    app.run()
}

#[derive(Component)]
struct MainCamera;

fn main_ui(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    objs: Query<(Entity, &SpaceObject)>,
) {
    egui::Window::new("Solar System")
        .default_width(300.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label(RichText::new("The Solar System").size(20.0));

            ui.separator();

            ui.label("Click on a planet to zoom in and get more information on it.");
            ui.label("You can click both the planet in the simulation and the planet in the list to zoom in.");
            ui.separator();
            ui.label("Press 'Esc' to reset the camera.");
            ui.separator();
            egui::Grid::new("planets").show(ui, |ui| {
            for obj in enum_iterator::all::<SpaceObject>() {
                if obj.orbits() == SpaceObject::Sun {
                    ui.end_row();
                }

                if ui.small_button(obj.name()).clicked() {
                    for (entity, &other_obj) in objs.iter() {
                        let mut entity = commands.entity(entity);

                        if other_obj == obj {
                           entity.insert(CurrentObject);
                        } else {
                            entity.remove::<CurrentObject>();
                        }
                    }
                }
            }});
        });
}

fn scientific_notation(num: f32) -> String {
    let mut num = num;
    let mut exp = 0;

    while num >= 10.0 {
        num /= 10.0;
        exp += 1;
    }

    while num < 1.0 {
        num *= 10.0;
        exp -= 1;
    }

    format!("{num}x10^{exp}")
}

fn obj_info_ui(mut egui_ctx: ResMut<EguiContext>, obj: Query<&SpaceObject, With<CurrentObject>>) {
    if let Ok(obj) = obj.get_single() {
        egui::Window::new(obj.name())
            .default_width(300.0)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.label(obj.name());
                ui.separator();
                ui.label(format!("Mass: {} kg", scientific_notation(obj.mass())));
                ui.label(format!("Diameter: {} km", obj.radius() * 2.0));
                ui.label(format!(
                    "Distance from what it orbits: {} AU",
                    obj.distance()
                ));
                ui.label(format!("Number of moons: {}", obj.num_moons()));
                ui.label(format!("Average temperature: {}°C", obj.temperature()));
                ui.label(format!(
                    "Period of revolution: {} days",
                    obj.period_of_revolution()
                ));
                ui.label(format!(
                    "Period of rotation: {} days",
                    obj.period_of_rotation()
                ));
                ui.label(format!("Orbits: {}", obj.orbits().name()));
                ui.separator();
                ui.label(format!("Fun fact: {}", obj.fun_fact()));
            });
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MainCamera,
        Rig::builder()
            .with(Position::new(DEFAULT_CAMERA_POSITION))
            .with(Smooth::new_position(1.0).predictive(true))
            .with(Smooth::new_position(2.5))
            .with(
                LookAt::new(glam::Vec3::ZERO)
                    .tracking_smoothness(1.25)
                    .tracking_predictive(true),
            )
            .build(),
    ));

    commands.spawn((
        MainCamera,
        Camera3dBundle::default(),
        PickingCameraBundle::default(), // <- Sets the camera to use for picking.
        Fxaa {
            edge_threshold: Sensitivity::High,
            ..default()
        },
    ));

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
                radius: obj.scaled_radius(),
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
                        let mut t = Transform::from_xyz(obj.scaled_distance(), 0.0, 0.0);
                        // flip the planet so it's not sideways
                        t.rotate_x(90.0_f32.to_radians());
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
    for (mut transform, planet, orbit) in planet_q
        .iter_mut()
        .filter(|(_, &o)| o != SpaceObject::Sun)
        .filter_map(|(t, p)| {
            Some((
                *t,
                p,
                main_planets
                    .iter()
                    .find_map(|(_, orbit)| if p.orbits() == *orbit { Some(*t) } else { None })?,
            ))
        })
    {
        transform.translate_around(
            orbit.translation,
            Quat::from_rotation_y(planet.orbital_velocity() * time.delta_seconds()),
        );
    }
}

fn escape(
    mut commands: Commands,
    obj: Query<Entity, With<CurrentObject>>,
    kbd: ResMut<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::Escape) {
        info!("Escape pressed");

        if let Ok(planet) = obj.get_single() {
            commands.entity(planet).remove::<CurrentObject>();
        }
    }
}

fn reset_camera(
    no_planet: Query<Entity, With<CurrentObject>>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    if no_planet.is_empty() {
        *cam.single_mut() = Transform::from_xyz(
            DEFAULT_CAMERA_POSITION.x,
            DEFAULT_CAMERA_POSITION.y,
            DEFAULT_CAMERA_POSITION.z,
        )
        .looking_at(Vec3::ZERO, Vec3::Y);
    }
}

// when a planet is selected, show information about it, zoom in on it, and change the camera's orbit
fn object_selected(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    current_planet: Query<Entity, With<CurrentObject>>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(entity)) = event {
            info!(?entity, "Selected planet");
            if let Ok(planet) = current_planet.get_single() {
                commands.entity(planet).remove::<CurrentObject>();
            }
            commands.entity(*entity).insert(CurrentObject);
        }
    }
}

fn lock_to_object(
    planet: Query<(&SpaceObject, &Transform), With<CurrentObject>>,
    mut rig: Query<&mut Rig>,
) {
    if let Ok((planet, transform)) = planet.get_single() {
        let mut rig = rig.single_mut();
        rig.driver_mut::<LookAt>().target = transform.transform_2_dolly().position;
        let mut cam_pos = glam::Vec3::Z * planet.scaled_radius() * 3.0;

        if transform.translation.z < 0.0 {
            cam_pos.z = -cam_pos.z;
        }

        rig.driver_mut::<Position>().position = transform.transform_2_dolly().position + cam_pos;
    }
}
