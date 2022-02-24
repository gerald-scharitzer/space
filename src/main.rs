fn main() {
    println!("Space, the final frontier");
    struct Planet {
        name: String,
        distance: f64, // semi-major axis in metres
        period: f64, // siderial orbital period in seconds
        mean_anomaly: f64 // rad
    }

    const SECONDS_PER_DAY_F64: f64 = 24.0 * 60.0 * 60.0;

    let mut earth = Planet {
        name: "Earth".to_string(),
        distance: 149_598_023_000.0, // greater than the astronomical unit
        period: 365.256_363_004 * SECONDS_PER_DAY_F64,
        mean_anomaly: 0.0
    };

    let mut mars = Planet {
        name: "Mars".to_string(),
        distance: 227_939_200_000.0,
        period: 686.98 * SECONDS_PER_DAY_F64,
        mean_anomaly: 0.0
    };

    let mut planets = vec![earth, mars];

    for time in 1..17 {
        for planet in &mut planets {
            planet.mean_anomaly += 0.0625;
        }
        println!("{} {}", planets[0].mean_anomaly, planets[1].mean_anomaly);
    }
}
