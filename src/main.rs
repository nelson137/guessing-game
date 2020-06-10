use std::cmp::Ordering;
use std::io::{self, Write};
use rand::Rng;

#[allow(dead_code)]
enum Style {
    Fg = 3,
    Bg = 4,
}

#[allow(dead_code)]
enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

fn reset_screen() {
    print!("\x1b[H\x1b[J\r");
}

macro_rules! repeat_char {
    ($c: literal, $n: expr) => {
        (0..$n).map(|_| $c).collect::<String>();
    }
}

fn colorize(style: Style, color: Color, text: &str) -> String {
    format!(
        "\x1b[{}{}m{}\x1b[0m",
        style as u8,
        color as u8,
        text
    ).to_string()
}

fn fg(color: Color, text: &str) -> String {
    colorize(Style::Fg, color, text)
}

struct Span {
    min: u32,
    max: u32
}

impl Span {
    fn range(&self) -> u32 {
        self.max - self.min
    }
}

/**
 *  0    4   8   12
 * [----(-----)----]
 *  ^    ^   ^    ^
 *  |    |   |    max
 *  |    |   end selection
 *  |    begin selection
 *  min
 *
 *  bounds = (min, max)
 *  selected = (begin, end)
 */
fn print_view(bounds: &Span, selected: &Span) {
    let before_range = repeat_char!('=', selected.min-bounds.min);
    let range = fg(
        Color::Green,
        &repeat_char!('=', selected.range()+1));
    let after_range = repeat_char!('=', bounds.max-selected.max);
    println!(
        "{} [{}{}{}] {}",
        bounds.min,
        before_range, range, after_range,
        bounds.max);
}

fn main() {
    let min: u32 = 1;
    let max: u32 = 50;
    let full_range = Span { min, max };
    let rand_num: u32 = rand::thread_rng().gen_range(min, max+1);

    let mut selection = Span { min, max };
    let mut hint: String = "/".to_string();

    loop {
        reset_screen();

        println!("Guess the number I'm thinking of... {}", rand_num);
        print!("\n");

        print_view(&full_range, &selection);
        println!("Hint: {}", hint);
        print!("\n");

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                hint = fg(Color::Red, "Invalid: Not a number");
                continue;
            }
        };

        if guess > max || guess < min {
            hint = fg(Color::Red, "Out of bounds");
            continue;
        }

        match guess.cmp(&rand_num) {
            Ordering::Less => {
                hint = fg(Color::Yellow, "Too small");
                selection.min = guess + 1;
            },
            Ordering::Greater => {
                hint = fg(Color::Yellow, "Too big");
                selection.max = guess - 1;
            },
            Ordering::Equal => {
                println!("\n{}", fg(Color::Green, "You win!"));
                break;
            }
        }
    }
}
