fn main() {
    let initial_value: i32 = 0;
    const SIZE: usize = 3;

    let array = [initial_value; SIZE];
    let mut array_2d = [[initial_value; SIZE]; SIZE];

    array_2d[1][2] = 1;

    println!("Array: {:?}", array);
    println!("2D Array: {:?}", array_2d);

    let tuple: (char, u8, f64) = ('x', 3, 0.14);
    let (a, b, c) = tuple;

    println!("Tuple: {:?}", tuple);
    println!("Tuple values: {}, {}, {}", a, b, c);
}
