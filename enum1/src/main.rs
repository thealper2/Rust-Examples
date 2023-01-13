#[allow(dead_code)]
#[allow(unused_variables)]

enum Color {
    Red,
    Blue,
    Green,
    RGBColor(u8, u8, u8), // tuple
    Cmyk{cyan:u8, magenta:u8, yellow:u8, black:u8} // Struct 
}

fn main() {
    let c = Color::RGBColor(10, 20, 30);

    match c {
        Color::Blue => println!("b"),
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::RGBColor(0, 0, 0) | Color::Cmyk {cyan:_, magenta:_, yellow:_, black:255} => println!("black"),
        Color::RGBColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _=> println!("the other color")
    }
}
