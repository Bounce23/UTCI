use crate::error;
use crate::heat_indices::calculate_polynomial_regression;

/// This function calculates the temperature 
/// equivalent to Universal Thermal Climate 
/// index.
pub fn calculate_utci(
    air_temperature: f32,
    radiant_temperature: f32,
    wind_speed: f32,
    relative_humidity: f32,
) -> f32 {
    let output = calculate_polynomial_regression(
        air_temperature,
        radiant_temperature,
        wind_speed,
        relative_humidity,
    );

    output
    
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utci() {

        let output = calculate_utci(32.0, 28.0, 2.0, 90.0);
        println!("{:?}", output);
        assert_eq!(output, 34.993866);
        
    }

    #[test]
    fn test_utci_2() {

        let output = calculate_utci(35.0, 31.0, 3.4, 87.9);
        println!("{:?}", output);
        
    }
}