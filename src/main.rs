fn main() {
    println!("Space, the final frontier");
    struct Planet {
        name: String,
        distance: f64, // metres
        period: f64, // seconds
        mean_anomaly: f64 // rad
    }
    let mut earth = Planet {
        name: "Earth".to_string(),
        distance: 149_597_870_700.0, // astronomical unit
        period: 365.256 * 24.0 * 60.0 * 60.0,
        mean_anomaly: 0.0
    };
    for time in 1..17 {
        earth.mean_anomaly += 0.0625;
        println!("{}", earth.mean_anomaly);
    }
}
