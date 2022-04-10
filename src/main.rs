use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Space, the final frontier");
    struct Planet {
        name: String,
        distance: f64, // semi-major axis in metres
        period: f64, // siderial orbital period in seconds
        mean_anomaly: f64 // rad
    }

    const SECONDS_PER_DAY_F64: f64 = 24.0 * 60.0 * 60.0;

    let mut planets: Vec<Arc<Mutex<Planet>>> = vec![];

    let mut planet = Planet {
        name: "Earth".to_string(),
        distance: 149_598_023_000.0, // greater than the astronomical unit
        period: 365.256_363_004 * SECONDS_PER_DAY_F64,
        mean_anomaly: 0.0
    };
    planets.push(Arc::new(Mutex::new(planet)));

    planet = Planet {
        name: "Mars".to_string(),
        distance: 227_939_200_000.0,
        period: 686.98 * SECONDS_PER_DAY_F64,
        mean_anomaly: 0.0
    };
    planets.push(Arc::new(Mutex::new(planet)));

    let mut handles = vec![];

    for planet_arc_mutex in planets.iter() {
        let planet_mutex_clone = planet_arc_mutex.clone();
        let handle = thread::spawn(move || {
            let mut planet = planet_mutex_clone.lock().unwrap();
            for _ in 0..16 {
                planet.mean_anomaly += 0.0625;
                println!("{} {}", planet.name, planet.mean_anomaly);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for planet_arc_mutex in planets.iter() {
        let planet = planet_arc_mutex.lock().unwrap();
        println!("{}", planet.name);
    }
}
