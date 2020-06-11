use std::cmp::Ordering;
use std::io::{self, Write};
use rand::Rng;

#[macro_use]
mod color;
use color::{Color, Style};

mod span;
use span::Span;

mod view;

macro_rules! reset_screen {
    () => ( print!("\x1b[H\x1b[J\r") )
}

fn main() {
    let min: i32 = 1;
    let max: i32 = 50;
    let full_range = Span { min, max };
    let rand_num: i32 = rand::thread_rng().gen_range(min, max+1);

    let n_guesses = 5;
    let mut guesses_left = n_guesses;
    let mut input: String;
    let mut guess: i32 = std::i32::MIN;

    let mut range = full_range.clone();
    let mut hint: String = "/".to_string();

    loop {
        reset_screen!();

        println!("Guess the number I'm thinking of... {}", rand_num);
        print!("\n");

        println!("Number of guesses remaining: {}", guesses_left);
        print!("\n");

        view::print(&full_range, &range);
        print!("\n");

        println!("Hint: {}", hint);
        print!("\n");

        if guess == rand_num {
            println!("{}", fg!(Color::Green, "You win!"));
            break;
        } else if guesses_left == 0 {
            println!("{}", fg!(Color::Red, "You lose :("));
            println!("The number was {}.", rand_num);
            break;
        }

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        guess = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                hint = fg!(Color::Red, "Invalid: Not a number");
                continue;
            }
        };

        if range.contains(guess) == false {
            hint = fg!(Color::Red, "Out of bounds");
            continue;
        }

        guesses_left -= 1;

        hint = match guess.cmp(&rand_num) {
            Ordering::Less => {
                range.min = guess + 1;
                fg!(Color::Yellow, "Too small")
            },
            Ordering::Greater => {
                range.max = guess - 1;
                fg!(Color::Yellow, "Too big")
            },
            Ordering::Equal => {
                fg!(Color::Green, "Correct")
            }
        }
    }
}
