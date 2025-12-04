use std::fs;


fn process_input(input: &String) -> i64 {
	let total_positions = 100;
	let mut dial = 50;
	let mut count = 0;

	for line in input.lines() {
		let side = &line[0..1];
		let number = (&line[1..]).parse::<i32>().expect("Expect that second part of input is a number");
		let cicles = number / total_positions;
		let diff = number % total_positions;
		let inital_dial = dial;
		if side == "R" {
			dial = dial + diff;
		} else if side == "L" {
			dial = dial - diff;
		}
		if dial > 100 || (dial < 0 && inital_dial != 0) {
			count += 1;
		}
		dial = dial.rem_euclid(total_positions);
		println!("\nAdding {cicles} from {line}");
		println!("Partial dial: {dial}");
		count += cicles;
		if dial == 0 && diff != 0 {
			count += 1;
		}
		println!("Partial count: {count}");
	}
	count.into()
}

fn main() {
	// Read input file
	let input = fs::read_to_string("./input.txt")
		.expect("Expect a input file on this folder");
	let result = process_input(&input);
	println!("Final result: {result}");
}
