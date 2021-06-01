//! Module containing helper functions
//! to calulcuate utci

// Used to calculate the vapour pressure
// in the air based on temperature and
/// humidity
pub fn calculate_vapour_pressure(
    air_temperature: f32,
    relative_humidity: f32,
) -> f32 {
    let g: [f32; 7] = [
        -2836.5744,
        -6028.076559,
        19.54263612,
        -0.02737830188,
        0.000016261698,
        (f32::powi(70.229056, -10)),
        (f32::powi(-18.680009, -13)),
    ];

    let kelvin_temperature = air_temperature + 273 as f32;

    let mut conversion =
        2.7150305f32.log(kelvin_temperature + 1f32);

    for i in 0..7 {
        conversion = g[i]
            * kelvin_temperature
                .powf((i as isize - 2) as f32);
    }

    let water_vapour = exp(conversion) * 0.01;

    water_vapour * (relative_humidity / 100f32)
}

// Exponentiation function
fn exp(x: f32) -> f32 {
    const MAX_ITER: i32 = 200;
    let mut sum = 1.0;
    let mut term = 1.0;

    for n in 1..MAX_ITER {
        term *= x / n as f32;
        sum += term;
    }

    return sum;
}

/// Used to find an approximation of the 6th
/// order polynomial regression model for
/// quatifying utci equivaent temperature
pub fn calculate_polynomial_regression(
    air_temperature: f32,
    radiant_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> f32 {
    let temp_diff = radiant_temperature - air_temperature;
    // convert to kpas
    let vapour_pressure = calculate_vapour_pressure(
        air_temperature,
        relative_humidity,
    ) / 10f32;

    let approximation = air_temperature
        + 0.607562052
        + (-0.0227712343) * air_temperature
        + (f32::powi(80.6470249, -4))
            * air_temperature
            * air_temperature
        + (f32::powi(-15.4271372, -4))
            * air_temperature
            * air_temperature
            * air_temperature
        + (f32::powi(-32.4651735, -4))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
        + (f32::powi(73.2602852, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
        + (f32::powi(13.5959073, -9))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
        + (-2.25836520) * wind_speed
        + 0.0880326035 * air_temperature * wind_speed
        + 0.00216844454
            * air_temperature
            * air_temperature
            * wind_speed
        + (f32::powi(-15.3347087, -5))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
        + (f32::powi(-57.2983704, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
        + (f32::powi(-25.5090145, -9))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
        + (-0.751269505) * wind_speed * wind_speed
        + (-0.00408350271)
            * air_temperature
            * wind_speed
            * wind_speed
        + (f32::powi(-52.1670675, -5))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
        + (f32::powi(19.4544667, -6))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
        + (f32::powi(11.4099531, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
        + 0.158137256
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(-65.7263143, -5))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(22.2697524, -7))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(-41.6117031, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
        + (-0.0127762753)
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(96.6891875, -6))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(25.2785852, -9))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(45.6306672, -4))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(-17.4202546, -7))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + (f32::powi(-59.1491269, -6))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
        + 0.398374029 * temp_diff
        + (f32::powi(18.3945314, -4))
            * air_temperature
            * temp_diff
        + (f32::powi(-17.3754510, -4))
            * air_temperature
            * air_temperature
            * temp_diff
        + (f32::powi(-76.0781159, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
        + (f32::powi(37.7830287, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
        + (f32::powi(54.3079673, -10))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
        + (-0.0200518269) * wind_speed * temp_diff
        + (f32::powi(89.2859837, -4))
            * air_temperature
            * wind_speed
            * temp_diff
        + (f32::powi(34.5433048, -6))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
        + (f32::powi(-37.7925774, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
        + (f32::powi(-16.9699377, -9))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
        + (f32::powi(16.9992415, -4))
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(-49.9204314, -5))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(24.7417178, -7))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(10.7596466, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(84.9242932, -5))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(13.5191328, -6))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(-62.1531254, -9))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(-49.9410301, -6))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(-18.9489258, -8))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(81.5300114, -8))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
        + (f32::powi(75.5043090, -4))
            * temp_diff
            * temp_diff
        + (f32::powi(-56.5095215, -5))
            * air_temperature
            * temp_diff
            * temp_diff
        + (f32::powi(-45.2166564, -7))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
        + (f32::powi(24.6688878, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
        + (f32::powi(24.2674348, -10))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
        + (f32::powi(15.4547250, -4))
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(52.4110970, -6))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-87.5874982, -8))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-15.0743064, -9))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-15.6236307, -5))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-13.3895614, -7))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(24.9709824, -9))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(65.1711721, -7))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(19.4960053, -9))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-10.0361113, -8))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
        + (f32::powi(-12.1206673, -5))
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-21.8203660, -7))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(75.1269482, -9))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(97.9063848, -11))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(12.5006734, -6))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-18.1584736, -9))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-35.2197671, -10))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-33.6514630, -8))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(13.5908359, -10))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(41.7032620, -10))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-13.0369025, -9))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(41.3908461, -10))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(92.2652254, -12))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-50.8220384, -9))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-22.4730961, -11))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(11.7139133, -10))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(66.2154879, -10))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(40.3863260, -13))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(19.5087203, -12))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + (f32::powi(-47.3602469, -12))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
        + 5.12733497 * vapour_pressure
        + (-0.312788561)
            * air_temperature
            * vapour_pressure
        + (-0.0196701861)
            * air_temperature
            * air_temperature
            * vapour_pressure
        + (f32::powi(99.9690870, -4))
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
        + (f32::powi(95.1738512, -6))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
        + (f32::powi(-46.6426341, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
        + 0.548050612 * wind_speed * vapour_pressure
        + (-0.00330552823)
            * air_temperature
            * wind_speed
            * vapour_pressure
        + (-0.00164119440)
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
        + (f32::powi(-51.6670694, -6))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
        + (f32::powi(95.2692432, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
        + (-0.0429223622)
            * wind_speed
            * wind_speed
            * vapour_pressure
        + 0.00500845667
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(10.0601257, -6))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(-18.1748644, -6))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(-12.5813502, -3))
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(-17.9330391, -4))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(23.4994441, -6))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(12.9735808, -4))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(12.9064870, -6))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (f32::powi(-22.8558686, -6))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
        + (-0.0369476348) * temp_diff * vapour_pressure
        + 0.00162325322
            * air_temperature
            * temp_diff
            * vapour_pressure
        + (f32::powi(-31.4279680, -5))
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
        + (f32::powi(25.9835559, -6))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
        + (f32::powi(-47.7136523, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
        + (f32::powi(86.4203390, -3))
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-68.7405181, -4))
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-91.3863872, -6))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(51.5916806, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-35.9217476, -5))
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(32.8696511, -5))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-71.0542454, -7))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-12.4382300, -5))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-73.8584400, -9))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(22.0609296, -7))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
        + (f32::powi(-73.2469180, -4))
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-18.7381964, -5))
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(48.0925239, -6))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-87.5492040, -8))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(27.7862930, -5))
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-50.6004592, -6))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(11.4325367, -7))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(25.3016723, -6))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-17.2857035, -8))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-39.5079398, -8))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-35.9413173, -7))
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(70.4388046, -7))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-18.9309167, -8))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-47.9768731, -7))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(79.6079978, -9))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(16.2897058, -9))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(39.4367674, -8))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-11.8566247, -9))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(33.4678041, -10))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (f32::powi(-11.5606447, -10))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
        + (-2.80626406) * vapour_pressure * vapour_pressure
        + 0.548712484
            * air_temperature
            * vapour_pressure
            * vapour_pressure
        + (-0.00399428410)
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-95.4009191, -4))
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(19.3090978, -5))
            * air_temperature
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
        + (-0.308806365)
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + 0.0116952364
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(49.5271903, -4))
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-19.0710882, -5))
            * air_temperature
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + 0.00210787756
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-69.8445738, -4))
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(23.0109073, -5))
            * air_temperature
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(41.7856590, -4))
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-12.7043871, -5))
            * air_temperature
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-30.4620472, -6))
            * wind_speed
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
        + 0.0514507424
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (-0.00432510997)
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(89.9281156, -5))
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-71.4663943, -7))
            * air_temperature
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-26.6016305, -4))
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(26.3789586, -4))
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-70.1199003, -6))
            * air_temperature
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-10.6823306, -4))
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(36.1341136, -6))
            * air_temperature
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(22.9748967, -7))
            * wind_speed
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(30.4788893, -4))
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-64.2070836, -5))
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(11.6257971, -6))
            * air_temperature
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(76.8023384, -6))
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-54.7446896, -7))
            * air_temperature
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-35.9937910, -8))
            * wind_speed
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-43.6497725, -6))
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(16.8737969, -7))
            * air_temperature
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(26.7489271, -8))
            * wind_speed
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(32.3926897, -9))
            * temp_diff
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
        + (-0.0353874123)
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.221201190)
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.0155126038
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-26.3917279, -4))
            * air_temperature
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.0453433455
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.00432943862)
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(14.5389826, -4))
            * air_temperature
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(21.7508610, -4))
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-66.6724702, -5))
            * air_temperature
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(33.3217140, -5))
            * wind_speed
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.00226921615)
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(38.0261982, -4))
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-54.5314314, -9))
            * air_temperature
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-79.6355448, -4))
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(25.3458034, -5))
            * air_temperature
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-63.1223658, -6))
            * wind_speed
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(30.2122035, -4))
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-47.7403547, -6))
            * air_temperature
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(17.3825715, -6))
            * wind_speed
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-40.9087898, -7))
            * temp_diff
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.614155345
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.0616755931)
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.00133374846
            * air_temperature
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.00355375387
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-51.3027851, -4))
            * air_temperature
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(10.2449757, -4))
            * wind_speed
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.00148526421)
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-41.1469183, -5))
            * air_temperature
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-68.0434415, -6))
            * wind_speed
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(-97.7675906, -6))
            * temp_diff
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.0882773108
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (-0.00301859306)
            * air_temperature
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.00104452989
            * wind_speed
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + (f32::powi(24.7090539, -4))
            * temp_diff
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
        + 0.00148348065
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure
            * vapour_pressure;

    approximation
}
