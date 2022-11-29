// https://www.codewars.com/kata/546e2562b03326a88e000020
/*
Welcome. In this kata, you are asked to square every digit of a number and concatenate them.

For example, if we run 9119 through the function, 811181 will come out, because 9^2 is 81 and 1^2 is 1.

Note: The function accepts an integer and returns an integer
*/

fn square_digits(num: u64) -> u64 {
    let mut result = String::new();
    for c in num.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        result.push_str(&(digit * digit).to_string());
    }
    result.parse::<u64>().unwrap()
}
