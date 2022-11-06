use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Lets play game!");

    let sec_number = rand::thread_rng().gen_range(1..=100);

    println!("Let me tell you, secret number is: {sec_number}");

    loop {
        println!("Now, input your number!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You geuss: {guess}");

        match guess.cmp(&sec_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
