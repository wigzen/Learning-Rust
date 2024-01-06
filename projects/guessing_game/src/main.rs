use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Wigzen's Guessing Game");

    let secert = rand::thread_rng().gen_range(1..=100);

    print!("{secert}");

    loop {
        let mut guess = String::new(); // muttable variable
        println!("Enter your Guess\n");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");
        println!("You Have guessed : {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secert) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Larger"),
            Ordering::Equal => {
                println!("Yoo hoo ");
                break;
            }
        }
    }
}
