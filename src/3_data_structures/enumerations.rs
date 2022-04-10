enum Color {
    Red,
    Green,
    Blue,
    // tuple
    RgbColor(u8, u8, u8),
    // struct
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

pub(crate) fn run() {
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 12,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        Color::CmykColor { black: 255, .. } => println!("black"),
        _ => (),
    }
}
