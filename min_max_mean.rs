fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut min: i32 = numbers[0];
    let mut max: i32 = numbers[0];
    let mut sum: i32 = 0;
    let mean: f64;

    for &number in numbers.iter() {
        if number < min {
            min = number;
        }
        else if number > max {
            max = number;
        }
        sum += number;
    }
    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(min, -18);
    assert_eq!(max, 56);
    assert_eq!(mean, 12.5);
}