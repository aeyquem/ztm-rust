// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
#[derive(Debug)]
struct Person {
    age: i8,
    // * The color and name should be stored as a String
    name: String,
    fav_color: String,
}

impl Person {
    fn print_info(&self) {
        println!(
            "{} is {}yo and the fav color is {} ",
            self.name, self.age, self.fav_color
        );
    }
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 14,
            name: "don".to_owned(),
            fav_color: "red".to_owned(),
        },
        Person {
            age: 4,
            name: "doña".to_owned(),
            fav_color: "blue".to_owned(),
        },
        Person {
            age: 20,
            name: "donña".to_owned(),
            fav_color: "green".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for p in &people {
        // * Use an if expression to determine which person's info should be printed
        if p.age > 10 {
            // * The name and colors should be printed using a function
            p.print_info();
        }
    }
}
