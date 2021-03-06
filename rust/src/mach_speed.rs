pub const GROUND_AIR_TEMP_C: f64 = 15.0;
pub const GROUND_AIR_TEMP_F: f64 = 59.0;

const KELVIN_TO_CELCIUS:  f64 = 273.15;
const KNOTS_TO_KM:        f64 = 0.5399568;
const THE_MAGIC_CONSTANT: f64 = 38.967854;

pub fn kmh_to_mach(km_per_hour: f64, outside_air_temp_c: f64) -> f64 {
    return (KNOTS_TO_KM * km_per_hour) / (THE_MAGIC_CONSTANT * (outside_air_temp_c * KELVIN_TO_CELCIUS).sqrt());
}

pub fn mileh_to_mach(miles_per_hour: f64, outside_air_temp_f: f64) -> f64 {
    return kmh_to_mach(mile_to_km(miles_per_hour), fahr_to_celc(outside_air_temp_f));
}

pub fn mach_to_kmh(mach: f64, outside_air_temp_c: f64) -> f64 {
    return (mach * (THE_MAGIC_CONSTANT * (outside_air_temp_c + KELVIN_TO_CELCIUS).sqrt())) / KNOTS_TO_KM;
}

pub fn mach_to_mileh(mach: f64, outside_air_temp_f: f64) -> f64 {
    return km_to_miles(mach_to_kmh(mach, fahr_to_celc(outside_air_temp_f)));
}

// -------------------- Private Functions --------------------

fn mile_to_km(miles: f64) -> f64 {
    return miles * 1.609344;
}

fn km_to_miles(km: f64) -> f64 {
    return km * 0.621371192;
}

fn fahr_to_celc(fahr: f64) -> f64 {
    return (fahr - 32.0) * 0.56;
}
