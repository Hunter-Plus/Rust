use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("How many squirrels do we have?");
    let squirrel_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your wild guess!");
    let mut guess = String::new();

    let mut valid_guess = 0;

    'guessing: loop {
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Wrong input!");

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                valid_guess += 1;
                num
            }
            // use '_' to catch all errors
            Err(_) => {
                println!("Numbers only, please!");
                continue;
            }
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&squirrel_number) {
            Ordering::Less => println!("We are many!"),
            Ordering::Greater => println!("No no no, we are tiny rodents!"),
            Ordering::Equal => {
                println!("Bingo!!!");
                println!("You've tried {} times!", valid_guess);
                break 'guessing;
            }
        }
    }
}
