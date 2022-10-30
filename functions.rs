fn main() {
    let value: i64 = 16;
    let result: i64 = square(value);

    println!("{} squared is {}", value, result);
    println!(
        "{} degrees Celsius is {} degrees Fahrenheit",
        value,
        celsius_to_fahrenheit(value as f64)
    );
}

fn square(n: i64) -> i64 {
    n * n
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}
