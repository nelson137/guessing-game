use crate::color::{Color, Style};
use crate::span::Span;

macro_rules! num_digits {
    ($num: expr) => ( $num.to_string().len() )
}

macro_rules! repeat_char {
    ($c: expr, $n: expr) => ( &(0..$n).map(|_| $c).collect::<String>() )
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
pub fn print(bounds: &Span, range: &Span) {
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
