use duplicate::duplicate;

#[derive(Debug)]
pub struct Duration {
    time: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { time: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEARS_IN_SECONDS: f64 = 31557600.0;

#[duplicate(
    planet_type convert_constant;
    [ Mercury ] [0.2408467];
    [ Venus ] [ 0.61519726 ];
    [ Mars ] [ 1.8808158 ];
    [ Earth ] [ 1.0 ];
    [ Jupiter ] [ 11.862615 ];
    [ Saturn ] [ 29.447498 ];
    [ Uranus ] [ 84.016846 ];
    [ Neptune ] [ 164.79132 ];
)]
impl Planet for planet_type {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = d.time / EARTH_YEARS_IN_SECONDS;

        earth_years / convert_constant
    }
}
