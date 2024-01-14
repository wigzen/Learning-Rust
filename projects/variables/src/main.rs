/* *
*  Variables are immutable (you can't change the value of variable) by default;
*  Use 'let' keyword to define variables
*  Use 'mut' keyword to make variables mutable;
*  Use 'const' keyword to make constant variable in which 'mut' keyword can not be used ;
*  Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime ;
*  Rust’s naming convention for constants is to use all uppercase with underscores between words
* */
fn main() {
    let mut num = 5;
    println!("{num}");
    num = 4;
    println!("{num}");
}
