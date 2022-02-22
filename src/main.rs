fn main() {
    println!("Space, the final frontier");
    struct Planet {
        name: String,
        distance: f64, // metres
        period: f64    // seconds
    }
    let earth = Planet {
        name: "Earth".to_string(),
        distance: 149_597_870_700.0, // astronomical unit
        period: 365.256 * 24.0 * 60.0 * 60.0
    };
}
