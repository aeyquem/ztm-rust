// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Cosa {
    quantity: i32,
    id: i32,
}

fn print_q(cosa: &Cosa) {
    println!("it has {:?}", cosa.quantity);
}

fn print_id(cosa: &Cosa) {
    println!("it has id: {:?}", cosa.id);
}
fn main() {
    let cosa = Cosa {
        quantity: 1,
        id: 313,
    };

    print_q(&cosa);
    print_id(&cosa);
}
