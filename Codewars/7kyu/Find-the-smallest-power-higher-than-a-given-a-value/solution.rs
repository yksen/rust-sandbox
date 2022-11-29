// https://www.codewars.com/kata/56ba65c6a15703ac7e002075
/*
Now, another case. We have the number 1245678.
We want to know the 5th power, closest and higher than that number. The value will be 1419857.

We need a function find_next_power ( findNextPower in JavaScript, CoffeeScript and Haskell), that receives two arguments,
a value val, and the exponent of the power, pow_, and outputs the value that we want to find.
*/

fn find_next_power(val: u64, pow_: u32) -> u64 {
    let mut base: u64 = 1;
    while base.pow(pow_) <= val { base += 1 }
    base.pow(pow_)
}