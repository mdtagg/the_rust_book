use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //ch_1();
    //ch_2();
}

fn ch_1() {
    println!("hello world");
}
fn ch_2() {
    fn guessing_game() {
        println!("Guess a number!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Your secret number is {secret_number}");
        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Cannot read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too Big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
    guessing_game()
}
