use std::cmp::Ordering;
use std::io::{self, Write};
use structopt::{StructOpt, clap::{Error, ErrorKind, Format}};
use rand::Rng;

#[macro_use]
mod color;
use color::{Color, Style};

mod span;
use span::Span;

mod view;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short="w", long, conflicts_with("range"))]
    fill_width: bool,

    #[structopt(
        short, long,
        multiple=false, number_of_values=2,
        value_names=&["min", "max"]
    )]
    range: Option<Vec<i32>>,
}

macro_rules! arg_error {
    ($app_name: expr, $arg_help: expr, $error: expr) => (
        Error::with_description(
            &format!(
                "The argument '{}' {}\n\n\
                 USAGE:\n    {} {}\n\n\
                 For more information try {}",
                Format::Warning($arg_help), $error,
                $app_name, $arg_help,
                Format::Good("--help")
            ),
            ErrorKind::ValueValidation
        ).exit()
    )
}

fn get_fill_width_max() -> i32 {
    let failed_err = Error::with_description(
        &format!(
            "Failed to get terminal width for argument '{}'",
            Format::Warning("--fill-width")
        ),
        ErrorKind::ValueValidation
    );
    match term_size::dimensions() {
        Some((width, _)) => {
            if width == 0 {
                failed_err.exit();
            }
            (width - num_digits!(width) - 5) as i32
        },
        None => failed_err.exit()
    }
}

fn main() {
    let args = Cli::from_args();
    let app = Cli::clap();
    let app_name = app.get_name();

    let (min, max): (i32, i32) = match args.range {
        Some(r) => (r[0], r[1]),
        None => (1, if args.fill_width { get_fill_width_max() } else { 50 })
    };

    if !(min < max) {
        arg_error!(app_name, "--range <min> <max>", "requires that min < max");
    }

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

        println!("Guess the number I'm thinking of...");
        print!("\n");

        println!(
            "Number of guesses remaining: {}",
            Format::Warning(guesses_left.to_string())
        );
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
