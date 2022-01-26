// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a: i128, b: i128) -> i128 {
	a + b
}

fn display(a: i128) {
	println!("{:?}", a);
}

fn main() {
	display(sum(2938432984232444444444444422211133355, 98324823598322333333333333322277722244));
}
