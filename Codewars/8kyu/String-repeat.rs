// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e
/*
Write a function that accepts an integer n and a string s as parameters,
and returns a string of s repeated exactly n times.

6, "I"     -> "IIIIII"
5, "Hello" -> "HelloHelloHelloHelloHello"
*/

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}
