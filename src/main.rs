use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut tries: i8 = 0;
    let mut lower_bound = 1;
    let mut upper_bound = 100;

    let secret_number = rand::thread_rng().gen_range(lower_bound, upper_bound + 1);
    //println!("The secret number is: {}", secret_number);
    println!("I'm thinking of a number between {} and {}.", lower_bound, upper_bound);
    println!("How many guesses do you want? No more than 100...");

    let mut max_tries= String::new();

    io::stdin().read_line(&mut max_tries)
        .expect("Failed to read line");

    let max_tries: i8 = match max_tries.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("I couldnt understand that, giving you 7 tries.");
            7
        }
    };

    let max_tries: i8 = match max_tries {
            _ if max_tries <= 1 => 1,
            _ if max_tries >= 100 => 100,
            _ => max_tries,
        };

    println!("ok, you get {} {}", max_tries, if max_tries == 1 { "try" } else { "tries" });

    loop {
        if tries >= max_tries {
            println!("Out of tries :( The number I was thinking of was {}", secret_number);
            break;
        } else {
            let tries_remaining = max_tries - tries;
            // only show this after the first guess
            if tries > 0 {
                println!("You have {} {} remaining.", tries_remaining, if tries_remaining == 1 { "try" } else { "tries" });
            }
            println!("Please input your guess. Its a number between {} and {}", lower_bound, upper_bound);
            tries += 1;
        }

        let mut guess = String::new();

        // not sure how to work with this as a result, also not sure how to make this fail to try things.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lower_bound = guess.max(lower_bound);
                println!("{} is too SMALL!", guess)
            }
            Ordering::Greater => {
                upper_bound = guess.min(upper_bound);
                println!("{} is too BIG!", guess)
            }
            Ordering::Equal => {
                println!("You win! and it only took you {} {}", tries, if tries == 1 { "try" } else { "tries" });
                break;
            }
        }
    }
}
