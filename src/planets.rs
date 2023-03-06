use bevy::prelude::Component;

const ASTRO_UNIT: f32 = 149_597_870.7;

const SUN_RADIUS: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum Planet {
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
    pub fn radius(&self) -> f32 {
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

    /// The average distance from the sun in astronomical units.
    pub fn distance(&self) -> f32 {
        match self {
            Planet::Sun => 0.0,
            Planet::Mercury => 0.387,
            Planet::Venus => 0.723,
            Planet::Earth => 1.0,
            Planet::Mars => 1.524,
            Planet::Jupiter => 5.203,
            Planet::Saturn => 9.539,
            Planet::Uranus => 19.18,
            Planet::Neptune => 30.06,
            Planet::Pluto => 39.53,
        }
    }

    /// The average orbital speed in kilometers per second.
    /// This is the speed at which the planet would travel if it were in a circular orbit.
    /// The actual speed of the planet is affected by its distance from the Sun.
    /// The speed is calculated using the formula: `sqrt(G * M / r)`.
    /// Where `G` is the gravitational constant, `M` is the mass of the Sun, and `r` is the distance from the Sun.
    pub fn orbital_speed(&self) -> f32 {
        // thx copilot :)
        const GRAV: f32 = 6.674_08e-11;
        const MASS_OF_SUN: f32 = 1.989_1e30;
        let distance_from_sun = self.distance() * ASTRO_UNIT;
        (GRAV * MASS_OF_SUN / distance_from_sun).sqrt()
    }

    /// The scale of the planet relative to the Sun.
    /// The Sun has a radius of 695,700 km.
    pub fn scaled_radius(&self) -> f32 {
        if *self == Self::Sun {
            SUN_RADIUS
        } else {
            self.radius() / ASTRO_UNIT * (Self::Sun.radius() / SUN_RADIUS)
        }
    }

    /// The distance from the Sun.
    pub fn scaled_distance(&self) -> f32 {
        self.distance() * (Self::Sun.radius() / SUN_RADIUS * 2.0)
    }
}
