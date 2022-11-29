// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe
/*
Create a function that takes an integer as an argument and returns "Even" for even numbers or "Odd" for odd numbers. 
*/

fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 { "Even" } else { "Odd" }
}