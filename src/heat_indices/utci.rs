use crate::error::Error;

use crate::heat_indices::{
    calculate_polynomial_regression,
    calculate_vapour_pressure,
};

/// Method for calculating experienced 
/// Heat Stress in the city of Groningen
pub fn experienced_heat_stress(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
    binnenstad: bool,
    park: bool,
    shade: bool,
) -> f32 {
    match (binnenstad, park, shade) {
        // 1.
        (true, false, true) => utci_1(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // 2.
        (true, false, false) => utci_2(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // 3.
        (false, true, true) => utci_3(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // 4.
        (false, true, false) => utci_4(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // 5.
        (false, false, true) => utci_5(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // 6.
        (false, false, false) => utci_6(
            air_temperature,
            wind_speed,
            relative_humidity,
        ).unwrap(),
        // catch-all
        _ => panic!("not supported"),
    }
}

// Function to calculate model 1 parameters
fn utci_1(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    let svf = calculate_svf_trees(13.4, 8.3, 12.6);

    let s_d = short_wave_densities(svf);

    let l_d = long_wave_densities_urban_trees(
        svf,
        air_temperature,
    );

    let q_cooling = compute_cooling_energy(
        air_temperature,
        relative_humidity,
        wind_speed,
    );

    let aggregate_radiation = s_d + l_d - q_cooling;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate model 2 parameters
fn utci_2(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    let svf = calculate_svf(13.4, 12.6);

    let s_d = short_wave_densities(svf);

    let l_d =
        long_wave_densities_urban(svf, air_temperature);

    let aggregate_radiation = s_d + l_d;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate model 3 parameters
fn utci_3(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    
    let svf = calculate_svf_trees(9.2, 8.3, 12.8);

    let s_d = short_wave_densities(svf);

    let l_d =
        long_wave_densities_park(svf, air_temperature);

    let q_cooling = compute_cooling_energy(
        air_temperature,
        relative_humidity,
        wind_speed,
    );

    let aggregate_radiation = s_d + l_d - q_cooling;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate model 4 parameters
fn utci_4(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    let svf = calculate_svf(9.2, 12.8);

    let s_d = short_wave_densities(svf);

    let l_d =
        long_wave_densities_park(svf, air_temperature);

    let aggregate_radiation = s_d + l_d;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate model 5 parameters
fn utci_5(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    let svf = calculate_svf_trees(9.2, 8.3, 12.8);

    let s_d = short_wave_densities(svf);

    let l_d = long_wave_densities_urban_trees(
        svf,
        air_temperature,
    );

    let q_cooling = compute_cooling_energy(
        air_temperature,
        relative_humidity,
        wind_speed,
    );

    let aggregate_radiation = s_d + l_d - q_cooling;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate model 6 parameters
fn utci_6(
    air_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> Result<f32, Error> {
    if air_temperature < -50.0 || air_temperature > 50.0 {
        return Err(Error::InvalidAirTemperature);
    }
    if relative_humidity < 0.0 || relative_humidity > 100.0 {
        return Err(Error::InvalidHumidity);
    }
    if wind_speed< 0.0 || wind_speed > 17.0 {
        return Err(Error::InvalidWindSpeed);
    }
    let svf = calculate_svf(9.2, 12.8);
    let s_d = short_wave_densities(svf);
    let l_d =
        long_wave_densities_urban(svf, air_temperature);

    let aggregate_radiation = s_d + l_d;

    let radiant_temperature =
        calculate_mean_radiant_temperature(
            aggregate_radiation,
        );

    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature.unwrap(),
        wind_speed,
        relative_humidity,
    );
    Ok(output)
}

// Function to calculate Sky View Factor
fn calculate_svf(
    building_height: f32,
    distance_between_buildings: f32,
) -> f32 {
    let svf = building_height
        / (0.5 * distance_between_buildings);
    let svf_output = svf.atan().cos();
    svf_output
}

// Function to calculate Sky View Factor 
// with trees present 
fn calculate_svf_trees(
    building_height: f32,
    tree_height: f32,
    distance_between_buildings: f32,
) -> f32 {
    let svf = building_height
        / (0.5 * distance_between_buildings);
    let svf_1 = tree_height / building_height;
    let svf_2 = svf * svf_1;
    let svf_output = svf_2.atan().cos();
    svf_output
}

// Function to calculate Mean Radiant Temperature
fn calculate_mean_radiant_temperature(
    aggregate_radiation: f32,
) -> Result<f32, Error> {
    if aggregate_radiation < -50.0 || aggregate_radiation > 50.0 {
        return Err(Error::InvalidRadiantTemperature);
    }
    let sigma = 0.0000000567;
    let denominator = 0.97 * sigma;
    let sum = aggregate_radiation / denominator;
    let aggregate = f32::powf(sum, 0.25);
    let output = aggregate - 273.15;
    Ok(output)
}

// Function to calculate short wave radiant densities
fn short_wave_densities(sky_view_factor: f32) -> f32 {
    let radiation = 1000f32;
    let radiation_coefficient = 0.7;

    let output = radiation
        * (1f32 - sky_view_factor)
        * radiation_coefficient;
    output
}

// Function to calculate long wave radiant densities
// in urban areas
fn long_wave_densities_urban(
    sky_view_factor: f32,
    air_temperature: f32,
) -> f32 {
    let kelvin_temperature = air_temperature + 273.15;
    let sigma = 0.0000000567;
    let l_u = 0.88f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_d = 0.8f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_s = 0.9f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);

    let output = (sky_view_factor * l_d)
        - ((1f32 - sky_view_factor) * l_u)
        + (0.5 * l_s);
    let waves = 0.9 * output;
    waves
}

// Function to calculate long wave radiant densities
// in urban areas with trees present
fn long_wave_densities_urban_trees(
    sky_view_factor: f32,
    air_temperature: f32,
) -> f32 {
    let kelvin_temperature = air_temperature + 273.15;
    let sigma = 0.0000000567;
    let l_t = 0.92f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_u = 0.88f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_d = 0.8f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_s = 0.9f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);

    let l_h_s = (sky_view_factor * l_d)
        - ((1f32 - sky_view_factor) * l_u)
        - (0.5 * l_s);
    let output =
        (l_h_s * (1f32 - 0.92)) + (1f32 - 0.92) * l_t;
    let waves = 0.9 * output;
    waves
}

// Function to calculate long wave radiant densities
// in open parks
fn long_wave_densities_park(
    sky_view_factor: f32,
    air_temperature: f32,
) -> f32 {
    let kelvin_temperature = air_temperature + 273.15;
    let sigma = 0.0000000567;
    let l_t = 0.92f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_u = 0.98f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_d = 0.8f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);
    let l_s = 0.92f32
        * sigma
        * f32::powi(kelvin_temperature, 4i32);

    let l_h_s = (sky_view_factor * l_d)
        - ((1f32 - sky_view_factor) * l_u)
        - (0.5 * l_s);
    let output =
        (l_h_s * (1f32 - 0.92)) + (1f32 - 0.92) * l_t;
    let waves = 0.9 * output;
    waves
}

// Function to calculate cooling energy from
// trees
fn compute_cooling_energy(
    air_temperature: f32,
    relative_humidity: f32,
    wind_speed: f32,
) -> f32 {
    let c_leaf = calculate_c_leaf(
        air_temperature,
        relative_humidity,
    );
    let c_air =
        calculate_c_air(air_temperature, relative_humidity);
    let numerator_1 = c_leaf - c_air;
    let denominator_1 = 0.125
        + calculate_aerodynamic_resistance(wind_speed);
    let sum_1 = numerator_1 / denominator_1;
    let sum_2 = 3600f32 / 3.73f32;
    let output = sum_1 * sum_2;
    let energy = (output * 2450f32) / 3600f32;
    energy
}

// Function to calculate transpiration of leaves
fn calculate_c_leaf(
    air_temperature: f32,
    relative_humidity: f32,
) -> f32 {
    let kelvin_temperature = air_temperature + 273.15;
    let saturation = calculate_vapour_pressure(
        air_temperature,
        relative_humidity,
    );
    let numerator = 18f32 * saturation;
    let denominator = 8.314 * kelvin_temperature;
    let output = numerator / denominator;
    output
}

// Function to calculate transpiration of air
fn calculate_c_air(
    air_temperature: f32,
    relative_humidity: f32,
) -> f32 {
    let kelvin_temperature = air_temperature + 273.15;
    let numerator = 18f32 * relative_humidity;
    let denominator = 8.314 * kelvin_temperature;
    let output = numerator / denominator;
    output
}

// Function to calculate aerodynamic resistance 
// of trees
fn calculate_aerodynamic_resistance(
    wind_speed: f32,
) -> f32 {
    let resistance = wind_speed / (0.7921);
    resistance
}
