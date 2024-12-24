// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use a struct to encapsulate the box characteristics
struct Box {
    color: Color,
    weight: i32,
    height: i32,
    width: i32,
    length: i32,
}

// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    Brown,
}

impl Box {
    // * Implement functionality on the box struct to create a new box
    fn create_box(color: Color, weight: i32, height: i32, width: i32, length: i32) -> Box {
        let new_box = Box {
            color,
            weight,
            height,
            width,
            length,
        };

        return new_box;
    }
    // * Implement functionality on the box struct to print the characteristics
    fn print_data(&self) {
        println!("color: {:?}", self.color);
        println!("weight: {:?}", self.weight);
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("length: {:?}", self.length);
    }
}

fn main() {
    let my_box = Box::create_box(Color::Brown, 13, 10, 5, 1);
    my_box.print_data();
}
