use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    const MAX_TRIES: i8 = 7;
    let mut tries = 0;
    let mut lower_bound = 1;
    let mut upper_bound = 100;

    let secret_number = rand::thread_rng().gen_range(lower_bound, upper_bound + 1);
    //println!("The secret number is: {}", secret_number);
    println!("I'm thinking of a number between {} and {}. Can you guess what it is?", lower_bound, upper_bound);

    loop {
        if tries >= MAX_TRIES {
            println!("Out of tries :(");
            break;
        } else {
            let tries_remaining = MAX_TRIES - tries;
            println!("You have {} {} remaining.", tries_remaining, if tries_remaining == 1 { "try" } else { "tries" });
            println!("Please input your guess. Its a number between {} and {}", lower_bound, upper_bound);
            tries += 1;
        }

        let mut guess = String::new();

        // not sure how to work with this as a result, also not sure how to make this fail to try things.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lower_bound = guess;
                println!("{} is too small!", guess)
            }
            Ordering::Greater => {
                upper_bound = guess;
                println!("{} is too big!", guess)
            }
            Ordering::Equal => {
                println!("You win! and it only took you {} {}", tries, if tries == 1 { "try" } else { "tries" });
                break;
            }
        }
    }
}
