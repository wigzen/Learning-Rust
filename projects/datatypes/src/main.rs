/**
 * There are two subsets of datatypes
 * (1) scalar
 *      i .integer
 *      ii.floating point
 *      iii.boolean
 *      iv .characters
 * (2) compound :Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
 *
* */
fn main() {
    let num: i32 = 2566;
    const CONST: f32 = 3.16;
    let is_something: bool = false;
    let string: char = 'V';
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    const WEEKDAYS: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
    let three_s = [3; 5];
    println!("Hello, world! {}", three_s[0]);
}
