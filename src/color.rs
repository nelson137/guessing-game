#[allow(dead_code)]
pub enum Style {
    Fg = 3,
    Bg = 4,
}

#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[macro_export]
macro_rules! colorize {
    ($style: expr, $color: expr, $text: expr) => ( format!(
        "\x1b[{}{}m{}\x1b[0m",
        $style as u8,
        $color as u8,
        $text
    ) )
}

#[macro_export]
macro_rules! fg {
    ($color: expr, $text: expr) => ( colorize!(Style::Fg, $color, $text) )
}
