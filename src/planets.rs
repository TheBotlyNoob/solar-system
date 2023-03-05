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
    /// The average distance from the sun in kilometers.
    pub fn distance(&self) -> f32 {
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
    pub fn speed(&self) -> f32 {
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
    pub fn scale(&self) -> f32 {
        self.radius() / 6_957.0
    }
}
