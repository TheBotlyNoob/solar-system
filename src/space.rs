use bevy::prelude::*;

const ASTRO_UNIT: f32 = 149_597_870.7;

const GRAV: f32 = 6.674_08e-11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, enum_iterator::Sequence)]
/// Space objects, including moons (a max of 5 per planet), planets, and the Sun.
pub enum SpaceObject {
    Sun,
    Mercury,
    Venus,

    Earth,
    EarthMoon,

    Mars,
    Phobos,
    Deimos,

    Jupiter,
    Metis,
    Adrastea,
    Amalthea,
    Thebe,
    Io,

    Saturn,
    Enceladus,
    Mimas,
    Tethys,
    Dione,
    Rhea,
    Titan,

    Uranus,
    Miranda,
    Ariel,
    Umbriel,
    Titania,
    Oberon,

    Neptune,
    Triton,
    Nereid,
    Proteus,
    Larissa,
    Halimede,

    Pluto,
    Charon,
    Nix,
    Hydra,
    Kerberos,
    Styx,
}

impl SpaceObject {
    /// The name of the object.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sun => "The Sun",

            Self::Mercury => "Mercury",

            Self::Venus => "Venus",

            Self::Earth => "Earth",
            Self::EarthMoon => "The Moon",

            Self::Mars => "Mars",
            Self::Phobos => "Phobos",
            Self::Deimos => "Deimos",

            Self::Jupiter => "Jupiter",
            Self::Metis => "Metis",
            Self::Adrastea => "Adrastea",
            Self::Amalthea => "Amalthea",
            Self::Thebe => "Thebe",
            Self::Io => "Io",

            Self::Saturn => "Saturn",
            Self::Enceladus => "Enceladus",
            Self::Mimas => "Mimas",
            Self::Tethys => "Tethys",
            Self::Dione => "Dione",
            Self::Rhea => "Rhea",
            Self::Titan => "Titan",

            Self::Uranus => "Uranus",
            Self::Miranda => "Miranda",
            Self::Ariel => "Ariel",
            Self::Umbriel => "Umbriel",
            Self::Titania => "Titania",
            Self::Oberon => "Oberon",

            Self::Neptune => "Neptune",
            Self::Triton => "Triton",
            Self::Nereid => "Nereid",
            Self::Proteus => "Proteus",
            Self::Larissa => "Larissa",
            Self::Halimede => "Halimede",

            Self::Pluto => "Pluto",
            Self::Charon => "Charon",
            Self::Nix => "Nix",
            Self::Hydra => "Hydra",
            Self::Kerberos => "Kerberos",
            Self::Styx => "Styx",
        }
    }

    /// The radius of the planet in kilometers.
    pub fn radius(self) -> f32 {
        match self {
            Self::Sun => 695_700.0,

            Self::Mercury => 2_439.7,

            Self::Venus => 6_051.8,

            Self::Earth => 6_371.0,
            Self::EarthMoon => 1_737.4,

            Self::Mars => 3_389.5,
            Self::Phobos => 11.1,
            Self::Deimos => 6.2,

            Self::Jupiter => 69_911.0,
            Self::Metis => 21.5,
            Self::Adrastea => 8.2,
            Self::Amalthea => 83.5,
            Self::Thebe => 49.3,
            Self::Io => 1_821.6,

            Self::Saturn => 58_232.0,
            Self::Enceladus => 252.1,
            Self::Mimas => 198.2,
            Self::Tethys => 533.0,
            Self::Dione => 561.4,
            Self::Rhea => 764.3,
            Self::Titan => 2_575.5,

            Self::Uranus => 25_362.0,
            Self::Miranda => 240.8,
            Self::Ariel => 578.9,
            Self::Umbriel => 584.7,
            Self::Titania => 788.9,
            Self::Oberon => 761.4,

            Self::Neptune => 24_622.0,
            Self::Triton => 1_353.4,
            Self::Nereid => 170.0,
            Self::Proteus => 210.0,
            Self::Larissa => 97.0,
            Self::Halimede => 31.0,

            Self::Pluto => 1_188.3,
            Self::Charon => 606.0,
            Self::Nix => 23.0,
            Self::Hydra => 30.0,
            Self::Kerberos => 12.0,
            Self::Styx => 10.0,
        }
    }

    /// The average distance from the planet it orbits in astronomical units.
    pub fn distance(self) -> f32 {
        match self {
            Self::Sun => 0.0,

            Self::Mercury => 0.387,

            Self::Venus => 0.723,

            Self::Earth => 1.0,
            Self::EarthMoon => 0.00257,

            Self::Mars => 1.524,
            Self::Phobos => 0.000039,
            Self::Deimos => 0.000157,

            Self::Jupiter => 5.203,
            Self::Metis => 0.00128,
            Self::Adrastea => 0.0015,
            Self::Amalthea => 0.0032,
            Self::Thebe => 0.00422,
            Self::Io => 0.00282,

            Self::Saturn => 9.537,
            Self::Enceladus => 0.00317,
            Self::Mimas => 0.0196,
            Self::Tethys => 0.0384,
            Self::Dione => 0.0563,
            Self::Rhea => 0.126,
            Self::Titan => 0.0847,

            Self::Uranus => 19.191,
            Self::Miranda => 0.00129,
            Self::Ariel => 0.00195,
            Self::Umbriel => 0.00266,
            Self::Titania => 0.00817,
            Self::Oberon => 0.0127,

            Self::Neptune => 30.069,
            Self::Triton => 0.00237,
            Self::Nereid => 0.036,
            Self::Proteus => 0.0077,
            Self::Larissa => 0.00073,
            Self::Halimede => 0.0379,

            Self::Pluto => 39.482,
            Self::Charon => 0.00157,
            Self::Nix => 0.002,
            Self::Hydra => 0.0045,
            Self::Kerberos => 0.00347,
            Self::Styx => 0.0078,
        }
    }

    /// The mass of the planet in kilograms.
    /// The Sun has a mass of 1.9891e30 kg.
    pub fn mass(self) -> f32 {
        match self {
            Self::Sun => 1.9891e30,

            Self::Mercury => 3.3011e23,

            Self::Venus => 4.8675e24,

            Self::Earth => 5.97237e24,
            Self::EarthMoon => 7.342e22,

            Self::Mars => 6.4171e23,
            Self::Phobos => 1.0659e16,
            Self::Deimos => 1.4762e15,

            Self::Jupiter => 1.8982e27,
            Self::Metis => 1.2e17,
            Self::Adrastea => 2.2e18,
            Self::Amalthea => 2.08e18,
            Self::Thebe => 4.3e19,
            Self::Io => 8.931938e22,

            Self::Saturn => 5.6834e26,
            Self::Enceladus => 1.08e20,
            Self::Mimas => 3.75e19,
            Self::Tethys => 6.17449e20,
            Self::Dione => 1.095452e21,
            Self::Rhea => 2.306518e21,
            Self::Titan => 1.3452e23,

            Self::Uranus => 8.68103e25,
            Self::Miranda => 6.59e19,
            Self::Ariel => 1.353e21,
            Self::Umbriel => 1.172e21,
            Self::Titania => 3.49e21,
            Self::Oberon => 3.014e21,

            Self::Neptune => 1.0241e26,
            Self::Triton => 2.14e22,
            Self::Nereid => 3.1e19,
            Self::Proteus => 5.37e19,
            Self::Larissa => 4.2e18,
            Self::Halimede => 4.0e18,

            Self::Pluto => 1.303e22,
            Self::Charon => 1.586e21,
            Self::Nix => 4.5e16,
            Self::Hydra => 4.2e16,
            Self::Kerberos => 1.65e16,
            Self::Styx => 7.5e15,
        }
    }

    /// The average orbital velocity in meters per second around [`Self::orbits`].
    pub fn orbital_velocity(self) -> f32 {
        if self == Self::Sun {
            return 0.0;
        }

        let distance_from = self.distance() * self.orbits().radius() * ASTRO_UNIT + self.radius();
        (GRAV * self.orbits().mass() / distance_from).sqrt() / 10_000.0
    }

    /// The object that this object orbits.
    /// The Sun orbits itself.
    pub fn orbits(self) -> Self {
        match self {
            Self::Sun => Self::Sun,

            Self::Mercury => Self::Sun,

            Self::Venus => Self::Sun,

            Self::Earth => Self::Sun,
            Self::EarthMoon => Self::Earth,

            Self::Mars => Self::Sun,
            Self::Phobos => Self::Mars,
            Self::Deimos => Self::Mars,

            Self::Jupiter => Self::Sun,
            Self::Metis => Self::Jupiter,
            Self::Adrastea => Self::Jupiter,
            Self::Amalthea => Self::Jupiter,
            Self::Thebe => Self::Jupiter,
            Self::Io => Self::Jupiter,

            Self::Saturn => Self::Sun,
            Self::Enceladus => Self::Saturn,
            Self::Mimas => Self::Saturn,
            Self::Tethys => Self::Saturn,
            Self::Dione => Self::Saturn,
            Self::Rhea => Self::Saturn,
            Self::Titan => Self::Saturn,

            Self::Uranus => Self::Sun,
            Self::Miranda => Self::Uranus,
            Self::Ariel => Self::Uranus,
            Self::Umbriel => Self::Uranus,
            Self::Titania => Self::Uranus,
            Self::Oberon => Self::Uranus,

            Self::Neptune => Self::Sun,
            Self::Triton => Self::Neptune,
            Self::Nereid => Self::Neptune,
            Self::Proteus => Self::Neptune,
            Self::Larissa => Self::Neptune,
            Self::Halimede => Self::Neptune,

            Self::Pluto => Self::Sun,
            Self::Charon => Self::Pluto,
            Self::Nix => Self::Pluto,
            Self::Hydra => Self::Pluto,
            Self::Kerberos => Self::Pluto,
            Self::Styx => Self::Pluto,
        }
    }

    /// Returns a constant fun fact about the given [`SpaceObject`].
    pub fn fun_fact(&self) -> &'static str {
        match self {
            SpaceObject::Sun => "The Sun is so big that about 1.3 million Earths could fit inside it!",
            SpaceObject::Mercury => "Mercury is the smallest planet in the Solar System, and it also has the most craters!",
            SpaceObject::Venus => "Venus is the hottest planet in the Solar System, with surface temperatures that can melt lead!",
            SpaceObject::Earth => "Earth is the only planet known to have liquid water on its surface, and it's also the only planet known to support life!",
            SpaceObject::EarthMoon => "The Moon is moving away from Earth at around 3.8 cm per year!",
            SpaceObject::Mars => "Mars is often called the \"Red Planet\" because of its reddish appearance, caused by iron oxide or rust on its surface!",
            SpaceObject::Phobos => "Phobos is the larger of Mars' two moons, and it's so close to the planet that it will eventually be torn apart by tidal forces!",
            SpaceObject::Deimos => "Deimos is the smaller of Mars' two moons, and it's named after the Greek god of terror!",
            SpaceObject::Jupiter => "Jupiter is the largest planet in the Solar System, and it also has the strongest magnetic field of any planet!",
            SpaceObject::Io => "Io is one of Jupiter's four largest moons, and it's the most volcanically active object in the Solar System!",
            SpaceObject::Saturn => "Saturn is famous for its beautiful rings, which are made up of ice and rock particles ranging in size from tiny grains to massive chunks!",
            SpaceObject::Uranus => "Uranus is the only planet in the Solar System that rotates on its side, with its north and south poles pointing almost directly at the Sun!",
            SpaceObject::Neptune => "Neptune is the farthest planet from the Sun, and it has the strongest winds of any planet in the Solar System, with gusts that can reach up to 1,200 miles per hour!",
            _ => "Sorry, no fun fact available for this planet yet!",
        }
    }

    /// The scale of the planet relative to the Sun.
    /// The Sun has a radius of 695,700 km.
    pub fn scaled_radius(self) -> f32 {
        if self == Self::Sun {
            self.radius() / 100.0
        } else {
            self.radius() / (Self::Sun.radius() / 100_000.0) // just make it a bit bigger
        }
    }

    /// The distance from the Sun.
    pub fn scaled_distance(self) -> f32 {
        self.distance() * (Self::Sun.radius() / 10.0)
    }

    // basic information to display on the planet info screen

    /// The number of moons orbiting this planet.
    /// The Sun has no moons.
    pub fn num_moons(self) -> usize {
        match self {
            Self::Sun => 0,
            Self::Mercury => 0,
            Self::Venus => 0,
            Self::Earth => 1,
            Self::Mars => 2,
            Self::Jupiter => 79,
            Self::Saturn => 62,
            Self::Uranus => 27,
            Self::Neptune => 14,
            Self::Pluto => 5,
            _ => 0,
        }
    }

    /// The average temperature of the planet in Celsius.
    pub fn temperature(self) -> f32 {
        match self {
            Self::Sun => 5778.0,
            Self::Mercury => 167.0,
            Self::Venus => 464.0,
            Self::Earth => 15.0,
            Self::EarthMoon => -20.0,
            Self::Mars => -63.0,
            Self::Phobos => -58.0,
            Self::Deimos => -40.0,
            Self::Jupiter => -108.0,
            Self::Metis => -100.0,
            Self::Adrastea => -100.0,
            Self::Amalthea => -100.0,
            Self::Thebe => -100.0,
            Self::Io => -143.0,
            Self::Saturn => -139.0,
            Self::Enceladus => -198.0,
            Self::Mimas => -201.0,
            Self::Tethys => -187.0,
            Self::Dione => -186.0,
            Self::Rhea => -196.0,
            Self::Titan => -179.0,
            Self::Uranus => -197.0,
            Self::Miranda => -187.0,
            Self::Ariel => -191.0,
            Self::Umbriel => -200.0,
            Self::Titania => -195.0,
            Self::Oberon => -197.0,
            Self::Neptune => -201.0,
            Self::Triton => -235.0,
            Self::Nereid => -220.0,
            Self::Proteus => -200.0,
            Self::Larissa => -197.0,
            Self::Halimede => -215.0,
            Self::Pluto => -229.0,
            Self::Charon => -229.0,
            Self::Nix => -230.0,
            Self::Hydra => -232.0,
            Self::Kerberos => -233.0,
            Self::Styx => -233.0,
        }
    }

    /// The period of revolution around the Sun in Earth days.
    /// The Sun has no period of revolution.
    pub fn period_of_revolution(self) -> f32 {
        match self {
            Self::Mercury => 88.0,
            Self::Venus => 225.0,
            Self::Earth => 365.0,
            Self::Mars => 687.0,
            Self::Jupiter => 4333.0,
            Self::Saturn => 10759.0,
            Self::Uranus => 30687.0,
            Self::Neptune => 60190.0,
            Self::Pluto => 90560.0,
            _ => 0.0,
        }
    }

    /// The period of rotation in Earth days.
    pub fn period_of_rotation(self) -> f32 {
        match self {
            Self::Sun => 25.38,
            Self::Mercury => 58.65,
            Self::Venus => 243.02,
            Self::Earth => 1.00,
            Self::EarthMoon => 27.32,
            Self::Mars => 1.03,
            Self::Phobos => 0.32,
            Self::Deimos => 1.26,
            Self::Jupiter => 0.41,
            Self::Metis => 0.30,
            Self::Adrastea => 0.30,
            Self::Amalthea => 0.50,
            Self::Thebe => 0.67,
            Self::Io => 1.77,
            Self::Saturn => 0.44,
            Self::Mimas => 0.94,
            Self::Enceladus => 1.37,
            Self::Tethys => 1.89,
            Self::Dione => 2.74,
            Self::Rhea => 4.52,
            Self::Titan => 15.95,
            Self::Uranus => 0.72,
            Self::Miranda => 1.41,
            Self::Ariel => 2.52,
            Self::Umbriel => 4.14,
            Self::Titania => 8.71,
            Self::Oberon => 13.46,
            Self::Neptune => 0.67,
            Self::Triton => -5.88,
            Self::Nereid => 360.13,
            Self::Proteus => 1.12,
            Self::Larissa => 0.55,
            Self::Halimede => 0.43,
            Self::Pluto => 6.39,
            Self::Charon => 6.39,
            Self::Nix => 1.83,
            Self::Hydra => 1.38,
            Self::Kerberos => 5.31,
            Self::Styx => 20.16,
        }
    }
}
