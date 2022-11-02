fn main() {
    let value: i64 = 16;
    let result: i64 = square(value);

    println!("{} squared is {}", value, result);
    println!(
        "{} degrees Celsius is {} degrees Fahrenheit",
        value,
        celsius_to_fahrenheit(value as f64)
    );
    println!("1 PLN is {} USD as of 02/11/2022", convert_currency(1.0, 0.21));
}

fn square(n: i64) -> i64 {
    n * n
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}

fn convert_currency(amount: f64, rate: f64) -> f64 {
    return amount * rate
}