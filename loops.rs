fn main() {
    let mut count: i8 = 0;
    let result: i8 = loop {
        println!("{}..", count);
        count += 1;
        if count == 4 {
            break count;
        }
    };
    println!("Result: {}", result);

    count = 0;
    while count < 3 {
        println!("{}", if count % 2 == 0 { "even" } else { "odd" });
        count += 1;
    }

    for number in 3..7 {
        println!("{}", number);
    }

    let message = ['h', 'e', 'l', 'l', 'o'];
    for (index, character) in message.iter().enumerate() {
        println!("{}: {}", index, character);
    }

    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.iter_mut() {
        for element in row.iter_mut() {
            *element *= *element;
            print!("{}\t", element);
        }
        println!();
    }
}
