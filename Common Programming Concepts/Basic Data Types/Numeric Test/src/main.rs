fn main() {
    let fahrenheit_degree = 451 as f32;
    let celsius_degree = (fahrenheit_degree - 32 as f32) * 0.5556;
    println!("{}°F is {:.1}°C", fahrenheit_degree, celsius_degree);
}
