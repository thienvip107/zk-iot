#![cfg_attr(feature = "guest", no_std)]

#[jolt::provable]
fn is_environment_healthy(temp: i32, co2_ppm: u32, aqi: u32) -> bool {
    // Check if all values are within healthy ranges
    let temp_healthy = temp >= 1500 && temp <= 2500;  // 15°C to 25°C
    let co2_healthy = co2_ppm < 1000;  // Below 1000 ppm is considered good
    let aqi_healthy = aqi <= 50;  // Good air quality

    temp_healthy && co2_healthy && aqi_healthy
}
