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

macro_rules! reset_screen {
    () => ( print!("\x1b[H\x1b[J\r") )
}

macro_rules! repeat_char {
    ($c: expr, $n: expr) => ( &(0..$n).map(|_| $c).collect::<String>() )
}

macro_rules! colorize {
    ($style: expr, $color: expr, $text: expr) => ( format!(
        "\x1b[{}{}m{}\x1b[0m",
        $style as u8,
        $color as u8,
        $text
    ) )
}

macro_rules! fg {
    ($color: expr, $text: expr) => ( colorize!(Style::Fg, $color, $text) )
}

macro_rules! num_digits {
    ($num: expr) => ( $num.to_string().len() )
}

#[derive(Clone)]
struct Span {
    min: u32,
    max: u32
}

impl Span {
    fn range(&self) -> usize {
        (self.max - self.min) as usize
    }

    fn contains(&self, num: u32) -> bool {
        self.min <= num && num <= self.max
    }
}

/**
 *  0    4   8   12
 * [----(-----)----]
 *  ^    ^   ^    ^
 *  |    |   |    max
 *  |    |   end range
 *  |    begin range
 *  min
 *
 *  bounds = (min, max)
 *  range = (begin, end)
 */
fn print_view(bounds: &Span, range: &Span) {
    // Number of characters before first '=' in view
    let prefix_w: usize = num_digits!(bounds.min) + 2;

    // Segment before range
    let seg_before: &str = repeat_char!('=', range.min-bounds.min);

    // Segment range
    let n_range: usize = range.range() + 1;
    let seg_range: String = fg!(Color::Green, repeat_char!('=', n_range));

    // Segment after range
    let seg_after: &str = repeat_char!('=', bounds.max-range.max);

    // Caret: beginning of seg_range
    let caret_begin_w: usize = prefix_w + seg_before.len() + 1;
    println!("{0:>1$}", range.min, caret_begin_w);
    println!("{0:>1$}", "v", caret_begin_w);

    // View
    println!(
        "{} [{}{}{}] {}",
        bounds.min,
        seg_before, seg_range, seg_after,
        bounds.max);

    // Caret: end of range
    let caret_end_w: usize = prefix_w + seg_before.len() + n_range;
    println!("{0:>1$}", "^", caret_end_w);
    println!("{0:>1$}", range.max, caret_end_w);
}

fn main() {
    let min: u32 = 1;
    let max: u32 = 50;
    let full_range = Span { min, max };
    let rand_num: u32 = rand::thread_rng().gen_range(min, max+1);

    let n_guesses = 5;
    let mut guesses_left = n_guesses;
    let mut input: String;
    let mut guess: u32 = min - 1;

    let mut range = full_range.clone();
    let mut hint: String = "/".to_string();

    loop {
        reset_screen!();

        println!("Guess the number I'm thinking of... {}", rand_num);
        print!("\n");

        println!("Number of guesses remaining: {}", guesses_left);
        print!("\n");

        print_view(&full_range, &range);
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
