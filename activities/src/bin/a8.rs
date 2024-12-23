// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Vanilla,
}
struct Drink {
    flavor: Flavor,
    volume: i16,
}

fn drink_info(drink: Drink) {
    println!("the drink is {:?}oz", drink.volume);
    match drink.flavor {
        Flavor::Vanilla => println!("flavor: vanilla"),
    }
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Vanilla,
        volume: 360,
    };
    drink_info(drink);
}
