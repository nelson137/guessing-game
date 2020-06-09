use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::cmp::Ordering;
use std::io::{self, Write};
use rand::Rng;

fn print_response(msg: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut colorspec = ColorSpec::new();

    // Set fg color
    colorspec.set_fg(Some(color));
    stdout.set_color(&colorspec)
        .expect("Unable to set foreground color");

    // Print message
    writeln!(&mut stdout, "{}", msg)
        .expect("Unable to print response");

    // Reset colors
    colorspec.clear();
    stdout.set_color(&colorspec)
        .expect("Unable to reset colors");
}

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number I'm thinking of...");
    println!("rand num = {}", rand_num);
    print!("\n");

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Unable to read line from stdin");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print_response("Invalid: not a number", Color::Red);
                continue;
            }
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => print_response("Too small.", Color::Yellow),
            Ordering::Greater => print_response("Too big.", Color::Yellow),
            Ordering::Equal => {
                print_response("You win!", Color::Green);
                break;
            }
        }
    }
}
