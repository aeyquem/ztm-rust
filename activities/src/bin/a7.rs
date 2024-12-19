// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color {
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Red => println!("red"),
    }
}

fn main() {
    let color = Color::Blue;

    print_color(color);
}
