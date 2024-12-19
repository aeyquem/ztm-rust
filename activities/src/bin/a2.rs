// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let a = 0;
    let b = 10;
    // * Use a function to add two numbers together
    let result = sum_two_nums(a, b);

    // * Use a function to display the result
    print_result(result);
}

fn print_result(result: i32) {
    // * Use the "{:?}" token in the println macro to display the result
    println!("result is {:?}", result);
}

fn sum_two_nums(a: i32, b: i32) -> i32 {
    return a + b;
}
