enum Color {
    WHITE,
    GRAY,
    BLACK,
    CUSTOM(String),
    HEX{ r: i8, g: i8, b: i8}
}

fn main() {
    let white = Color::WHITE;
    let orange = Color::CUSTOM("orange".to_string());
    let fav = Color::HEX{r: 3, g: 9, b: 27};

    print_color(white);
    print_color(orange);
    print_color(fav);
}


fn print_color(c: Color) {
    match c {
        Color::WHITE => println!("Color is white"),
        Color::GRAY => println!("Color is gray"),
        Color::BLACK => println!("Color is black"),
        Color::CUSTOM(color) => println!("Custom color: {}", color),
        _ => println!("Unknown")
    }
}