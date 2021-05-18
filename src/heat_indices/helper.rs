//! Module containing helper functions 
pub fn calculate_vapour_pressure(air_temperature: f32, relative_humidity: f32) -> f32 {

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

    let mut conversion = 2.7150305f32.log(kelvin_temperature + 1f32);

    for i in 0..6 {
        conversion = g[i] * kelvin_temperature.powf((i - 2) as f32);
    }
    
    let water_vapour = exp(conversion) * 0.01;

    water_vapour * (relative_humidity/100f32)
}

fn exp(x: f32) -> f32 {
    const MAX_ITER: i32 = 200; 
    let mut sum = 1.0;
    let mut term = 1.0;

    for n in 1..MAX_ITER {
    term *= x / n as f32;
    sum += term;
    };
 
    return sum
}

// Used to ...
pub fn calculate_polynomial_regression(air_temperature: f32, radiant_temperature: f32, wind_speed: f32, relative_humidity: f32) -> f32 {

    let temp_diff = radiant_temperature - air_temperature;
    // convert to kpas
    let vapour_pressure = calculate_vapour_pressure(air_temperature, relative_humidity) / 10f32;

    let approximation = air_temperature 
        + 0.607562052
        + (-0.0227712343) * air_temperature
        + (f32::powi(80.6470249, -4)) * air_temperature * air_temperature
        + (f32::powi(-15.4271372, -4)) * air_temperature * air_temperature * air_temperature
        + (f32::powi(-3.24651735, -4)) * air_temperature * air_temperature * air_temperature * air_temperature
        + (7.32602852 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature
        + (1.35959073 * (10 ** (-9))) * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature
        + (-2.25836520) * wind_speed
        + 0.0880326035 * air_temperature * wind_speed
        + 0.00216844454 * air_temperature * air_temperature * wind_speed
        + (-1.53347087 * (10 ** (-5))) * air_temperature * air_temperature * air_temperature * wind_speed
        + (-5.72983704 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * air_temperature * wind_speed
        + (-2.55090145 * (10 ** (-9))) * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature * wind_speed
        + (-0.751269505) * wind_speed * wind_speed
        + (-0.00408350271) * air_temperature * wind_speed * wind_speed
        + (-5.21670675 * (10 ** (-5))) * air_temperature * air_temperature * wind_speed * wind_speed
        + (1.94544667 * (10 ** (-6))) * air_temperature * air_temperature * air_temperature * wind_speed * wind_speed
        + (1.14099531 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * air_temperature * wind_speed * wind_speed
        + 0.158137256 * wind_speed * wind_speed * wind_speed
        + (-6.57263143 * (10 ** (-5))) * air_temperature * wind_speed * wind_speed * wind_speed
        + (2.22697524 * (10 ** (-7))) * air_temperature * air_temperature * wind_speed * wind_speed * wind_speed
        + (-4.16117031 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * wind_speed * wind_speed * wind_speed
        + (-0.0127762753) * wind_speed * wind_speed * wind_speed * wind_speed
        + (9.66891875 * (10 ** (-6))) * air_temperature * wind_speed * wind_speed * wind_speed * wind_speed
        + (2.52785852 * (10 ** (-9))) * air_temperature * air_temperature * wind_speed * wind_speed * wind_speed * wind_speed
        + (4.56306672 * (10 ** (-4))) * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed
        + (-1.74202546 * (10 ** (-7))) * air_temperature * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed
        + (-5.91491269 * (10 ** (-6))) * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed
        + 0.398374029 * temp_diff
        + (1.83945314 * (10 ** (-4))) * air_temperature * temp_diff
        + (-1.73754510 * (10 ** (-4))) * air_temperature * air_temperature * temp_diff
        + (-7.60781159 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * temp_diff
        + (3.77830287 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * air_temperature * temp_diff
        + (5.43079673 * (10 ** (-10))) * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature * temp_diff
        + (-0.0200518269) * wind_speed * temp_diff
        + (8.92859837 * (10 ** (-4))) * air_temperature * wind_speed * temp_diff
        + (3.45433048 * (10 ** (-6))) * air_temperature * air_temperature * wind_speed * temp_diff
        + (-3.77925774 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * wind_speed * temp_diff
        + (-1.69699377 * (10 ** (-9))) * air_temperature * air_temperature * air_temperature * air_temperature * wind_speed * temp_diff
        + (1.69992415 * (10 ** (-4))) * wind_speed * wind_speed * temp_diff
        + (-4.99204314 * (10 ** (-5))) * air_temperature * wind_speed * wind_speed * temp_diff
        + (2.47417178 * (10 ** (-7))) * air_temperature * air_temperature * wind_speed * wind_speed * temp_diff
        + (1.07596466 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * wind_speed * wind_speed * temp_diff
        + (8.49242932 * (10 ** (-5))) * wind_speed * wind_speed * wind_speed * temp_diff
        + (1.35191328 * (10 ** (-6))) * air_temperature * wind_speed * wind_speed * wind_speed * temp_diff
        + (-6.21531254 * (10 ** (-9))) * air_temperature * air_temperature * wind_speed * wind_speed * wind_speed * temp_diff
        + (-4.99410301 * (10 ** (-6))) * wind_speed * wind_speed * wind_speed * wind_speed * temp_diff
        + (-1.89489258 * (10 ** (-8))) * air_temperature * wind_speed * wind_speed * wind_speed * wind_speed * temp_diff
        + (8.15300114 * (10 ** (-8))) * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed * temp_diff
        + (7.55043090 * (10 ** (-4))) * temp_diff * temp_diff
        + (-5.65095215 * (10 ** (-5))) * air_temperature * temp_diff * temp_diff
        + (-4.52166564 * (10 ** (-7))) * air_temperature * air_temperature * temp_diff * temp_diff
        + (2.46688878 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * temp_diff * temp_diff
        + (2.42674348 * (10 ** (-10))) * air_temperature * air_temperature * air_temperature * air_temperature * temp_diff * temp_diff
        + (1.54547250 * (10 ** (-4))) * wind_speed * temp_diff * temp_diff
        + (5.24110970 * (10 ** (-6))) * air_temperature * wind_speed * temp_diff * temp_diff
        + (-8.75874982 * (10 ** (-8))) * air_temperature * air_temperature * wind_speed * temp_diff * temp_diff
        + (-1.50743064 * (10 ** (-9))) * air_temperature * air_temperature * air_temperature * wind_speed * temp_diff * temp_diff
        + (-1.56236307 * (10 ** (-5))) * wind_speed * wind_speed * temp_diff * temp_diff
        + (-1.33895614 * (10 ** (-7))) * air_temperature * wind_speed * wind_speed * temp_diff * temp_diff
        + (2.49709824 * (10 ** (-9))) * air_temperature * air_temperature * wind_speed * wind_speed * temp_diff * temp_diff
        + (6.51711721 * (10 ** (-7))) * wind_speed * wind_speed * wind_speed * temp_diff * temp_diff
        + (1.94960053 * (10 ** (-9))) * air_temperature * wind_speed * wind_speed * wind_speed * temp_diff * temp_diff
        + (-1.00361113 * (10 ** (-8))) * wind_speed * wind_speed * wind_speed * wind_speed * temp_diff * temp_diff
        + (-1.21206673 * (10 ** (-5))) * temp_diff * temp_diff * temp_diff
        + (-2.18203660 * (10 ** (-7))) * air_temperature * temp_diff * temp_diff * temp_diff
        + (7.51269482 * (10 ** (-9))) * air_temperature * air_temperature * temp_diff * temp_diff * temp_diff
        + (9.79063848 * (10 ** (-11)))
        * air_temperature
        * air_temperature
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        + (1.25006734 * (10 ** (-6))) * wind_speed * temp_diff * temp_diff * temp_diff
        + (-1.81584736 * (10 ** (-9))) * air_temperature * wind_speed * temp_diff * temp_diff * temp_diff
        + (-3.52197671 * (10 ** (-10)))
        * air_temperature
        * air_temperature
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        + (-3.36514630 * (10 ** (-8))) * wind_speed * wind_speed * temp_diff * temp_diff * temp_diff
        + (1.35908359 * (10 ** (-10)))
        * air_temperature
        * wind_speed
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        + (4.17032620 * (10 ** (-10)))
        * wind_speed
        * wind_speed
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        + (-1.30369025 * (10 ** (-9)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (4.13908461 * (10 ** (-10)))
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (9.22652254 * (10 ** (-12)))
        * air_temperature
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (-5.08220384 * (10 ** (-9)))
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (-2.24730961 * (10 ** (-11)))
        * air_temperature
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (1.17139133 * (10 ** (-10)))
        * wind_speed
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (6.62154879 * (10 ** (-10)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (4.03863260 * (10 ** (-13)))
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (1.95087203 * (10 ** (-12)))
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + (-4.73602469 * (10 ** (-12)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        + 5.12733497 * vapour_pressure
        + (-0.312788561) * air_temperature * vapour_pressure
        + (-0.0196701861) * air_temperature * air_temperature * vapour_pressure
        + (9.99690870 * (10 ** (-4))) * air_temperature * air_temperature * air_temperature * vapour_pressure
        + (9.51738512 * (10 ** (-6))) * air_temperature * air_temperature * air_temperature * air_temperature * vapour_pressure
        + (-4.66426341 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * air_temperature * air_temperature * vapour_pressure
        + 0.548050612 * wind_speed * vapour_pressure
        + (-0.00330552823) * air_temperature * wind_speed * vapour_pressure
        + (-0.00164119440) * air_temperature * air_temperature * wind_speed * vapour_pressure
        + (-5.16670694 * (10 ** (-6))) * air_temperature * air_temperature * air_temperature * wind_speed * vapour_pressure
        + (9.52692432 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * air_temperature * wind_speed * vapour_pressure
        + (-0.0429223622) * wind_speed * wind_speed * vapour_pressure
        + 0.00500845667 * air_temperature * wind_speed * wind_speed * vapour_pressure
        + (1.00601257 * (10 ** (-6))) * air_temperature * air_temperature * wind_speed * wind_speed * vapour_pressure
        + (-1.81748644 * (10 ** (-6))) * air_temperature * air_temperature * air_temperature * wind_speed * wind_speed * vapour_pressure
        + (-1.25813502 * (10 ** (-3))) * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (-1.79330391 * (10 ** (-4))) * air_temperature * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (2.34994441 * (10 ** (-6))) * air_temperature * air_temperature * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (1.29735808 * (10 ** (-4))) * wind_speed * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (1.29064870 * (10 ** (-6))) * air_temperature * wind_speed * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (-2.28558686 * (10 ** (-6))) * wind_speed * wind_speed * wind_speed * wind_speed * wind_speed * vapour_pressure
        + (-0.0369476348) * temp_diff * vapour_pressure
        + 0.00162325322 * air_temperature * temp_diff * vapour_pressure
        + (-3.14279680 * (10 ** (-5))) * air_temperature * air_temperature * temp_diff * vapour_pressure
        + (2.59835559 * (10 ** (-6))) * air_temperature * air_temperature * air_temperature * temp_diff * vapour_pressure
        + (-4.77136523 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * air_temperature * temp_diff * vapour_pressure
        + (8.64203390 * (10 ** (-3))) * wind_speed * temp_diff * vapour_pressure
        + (-6.87405181 * (10 ** (-4))) * air_temperature * wind_speed * temp_diff * vapour_pressure
        + (-9.13863872 * (10 ** (-6))) * air_temperature * air_temperature * wind_speed * temp_diff * vapour_pressure
        + (5.15916806 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * wind_speed * temp_diff * vapour_pressure
        + (-3.59217476 * (10 ** (-5))) * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (3.28696511 * (10 ** (-5))) * air_temperature * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (-7.10542454 * (10 ** (-7))) * air_temperature * air_temperature * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (-1.24382300 * (10 ** (-5))) * wind_speed * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (-7.38584400 * (10 ** (-9))) * air_temperature * wind_speed * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (2.20609296 * (10 ** (-7))) * wind_speed * wind_speed * wind_speed * wind_speed * temp_diff * vapour_pressure
        + (-7.32469180 * (10 ** (-4))) * temp_diff * temp_diff * vapour_pressure
        + (-1.87381964 * (10 ** (-5))) * air_temperature * temp_diff * temp_diff * vapour_pressure
        + (4.80925239 * (10 ** (-6))) * air_temperature * air_temperature * temp_diff * temp_diff * vapour_pressure
        + (-8.75492040 * (10 ** (-8))) * air_temperature * air_temperature * air_temperature * temp_diff * temp_diff * vapour_pressure
        + (2.77862930 * (10 ** (-5))) * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (-5.06004592 * (10 ** (-6))) * air_temperature * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (1.14325367 * (10 ** (-7))) * air_temperature * air_temperature * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (2.53016723 * (10 ** (-6))) * wind_speed * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (-1.72857035 * (10 ** (-8))) * air_temperature * wind_speed * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (-3.95079398 * (10 ** (-8))) * wind_speed * wind_speed * wind_speed * temp_diff * temp_diff * vapour_pressure
        + (-3.59413173 * (10 ** (-7))) * temp_diff * temp_diff * temp_diff * vapour_pressure
        + (7.04388046 * (10 ** (-7))) * air_temperature * temp_diff * temp_diff * temp_diff * vapour_pressure
        + (-1.89309167 * (10 ** (-8)))
        * air_temperature
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (-4.79768731 * (10 ** (-7))) * wind_speed * temp_diff * temp_diff * temp_diff * vapour_pressure
        + (7.96079978 * (10 ** (-9)))
        * air_temperature
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (1.62897058 * (10 ** (-9)))
        * wind_speed
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (3.94367674 * (10 ** (-8)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (-1.18566247 * (10 ** (-9)))
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (3.34678041 * (10 ** (-10)))
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (-1.15606447 * (10 ** (-10)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        + (-2.80626406) * vapour_pressure * vapour_pressure
        + 0.548712484 * air_temperature * vapour_pressure * vapour_pressure
        + (-0.00399428410) * air_temperature * air_temperature * vapour_pressure * vapour_pressure
        + (-9.54009191 * (10 ** (-4))) * air_temperature * air_temperature * air_temperature * vapour_pressure * vapour_pressure
        + (1.93090978 * (10 ** (-5))) * air_temperature * air_temperature * air_temperature * air_temperature * vapour_pressure * vapour_pressure
        + (-0.308806365) * wind_speed * vapour_pressure * vapour_pressure
        + 0.0116952364 * air_temperature * wind_speed * vapour_pressure * vapour_pressure
        + (4.95271903 * (10 ** (-4))) * air_temperature * air_temperature * wind_speed * vapour_pressure * vapour_pressure
        + (-1.90710882 * (10 ** (-5))) * air_temperature * air_temperature * air_temperature * wind_speed * vapour_pressure * vapour_pressure
        + 0.00210787756 * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + (-6.98445738 * (10 ** (-4))) * air_temperature * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + (2.30109073 * (10 ** (-5))) * air_temperature * air_temperature * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + (4.17856590 * (10 ** (-4))) * wind_speed * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + (-1.27043871 * (10 ** (-5))) * air_temperature * wind_speed * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + (-3.04620472 * (10 ** (-6))) * wind_speed * wind_speed * wind_speed * wind_speed * vapour_pressure * vapour_pressure
        + 0.0514507424 * temp_diff * vapour_pressure * vapour_pressure
        + (-0.00432510997) * air_temperature * temp_diff * vapour_pressure * vapour_pressure
        + (8.99281156 * (10 ** (-5))) * air_temperature * air_temperature * temp_diff * vapour_pressure * vapour_pressure
        + (-7.14663943 * (10 ** (-7))) * air_temperature * air_temperature * air_temperature * temp_diff * vapour_pressure * vapour_pressure
        + (-2.66016305 * (10 ** (-4))) * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (2.63789586 * (10 ** (-4))) * air_temperature * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (-7.01199003 * (10 ** (-6))) * air_temperature * air_temperature * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (-1.06823306 * (10 ** (-4))) * wind_speed * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (3.61341136 * (10 ** (-6))) * air_temperature * wind_speed * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (2.29748967 * (10 ** (-7))) * wind_speed * wind_speed * wind_speed * temp_diff * vapour_pressure * vapour_pressure
        + (3.04788893 * (10 ** (-4))) * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (-6.42070836 * (10 ** (-5))) * air_temperature * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (1.16257971 * (10 ** (-6))) * air_temperature * air_temperature * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (7.68023384 * (10 ** (-6))) * wind_speed * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (-5.47446896 * (10 ** (-7))) * air_temperature * wind_speed * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (-3.59937910 * (10 ** (-8))) * wind_speed * wind_speed * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (-4.36497725 * (10 ** (-6))) * temp_diff * temp_diff * temp_diff * vapour_pressure * vapour_pressure
        + (1.68737969 * (10 ** (-7)))
        * air_temperature
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        * vapour_pressure
        + (2.67489271 * (10 ** (-8)))
        * wind_speed
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        * vapour_pressure
        + (3.23926897 * (10 ** (-9)))
        * temp_diff
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        * vapour_pressure
        + (-0.0353874123) * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.221201190) * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.0155126038 * air_temperature * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure
        + (-2.63917279 * (10 ** (-4))) * air_temperature * air_temperature * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.0453433455 * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.00432943862) * air_temperature * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (1.45389826 * (10 ** (-4))) * air_temperature * air_temperature * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (2.17508610 * (10 ** (-4))) * wind_speed * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (-6.66724702 * (10 ** (-5))) * air_temperature * wind_speed * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (3.33217140 * (10 ** (-5))) * wind_speed * wind_speed * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.00226921615) * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (3.80261982 * (10 ** (-4))) * air_temperature * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (-5.45314314 * (10 ** (-9))) * air_temperature * air_temperature * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (-7.96355448 * (10 ** (-4))) * wind_speed * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (2.53458034 * (10 ** (-5))) * air_temperature * wind_speed * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (-6.31223658 * (10 ** (-6))) * wind_speed * wind_speed * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (3.02122035 * (10 ** (-4))) * temp_diff * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (-4.77403547 * (10 ** (-6))) * air_temperature * temp_diff * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (1.73825715 * (10 ** (-6))) * wind_speed * temp_diff * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure
        + (-4.09087898 * (10 ** (-7)))
        * temp_diff
        * temp_diff
        * temp_diff
        * vapour_pressure
        * vapour_pressure
        * vapour_pressure
        + 0.614155345 * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.0616755931) * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.00133374846 * air_temperature * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.00355375387 * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-5.13027851 * (10 ** (-4))) * air_temperature * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (1.02449757 * (10 ** (-4))) * wind_speed * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.00148526421) * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-4.11469183 * (10 ** (-5))) * air_temperature * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-6.80434415 * (10 ** (-6))) * wind_speed * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-9.77675906 * (10 ** (-6))) * temp_diff * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.0882773108 * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (-0.00301859306) * air_temperature * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.00104452989 * wind_speed * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + (2.47090539 * (10 ** (-4))) * temp_diff * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure
        + 0.00148348065 * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure * vapour_pressure


}