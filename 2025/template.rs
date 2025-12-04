use std::fs;


fn process_input(input: &String) -> i64 {
	//TODO write your code here
}

fn main() {
	// Read input file
	let input = fs::read_to_string("./input.txt")
		.expect("Expect a input file on this folder");
	let result = process_input(input);
	println!("Final result: {result}");
}
