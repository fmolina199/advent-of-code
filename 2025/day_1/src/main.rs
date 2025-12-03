use std::fs;

fn main() {
	// Read input file
	let input = fs::read_to_string("./input.txt")
		.expect("Expect a input file on this folder");
	
	// Starting point
	let total_positions = 100;
	let mut dial = 50;
	let mut count = 0;
	
	for line in input.lines() {
		let side = &line[0..1];
		let number = (&line[1..]).parse::<i32>().expect("Expect that second part of input is a number");
		if side == "R" {
			//println!("Adding {number}");
			dial = (dial + number).rem_euclid(total_positions);
		} else if side == "L" {
			//println!("Subtracting {number}");
			dial = (dial - number).rem_euclid(total_positions);
		}
		println!("Partial dial: {dial}");
		if dial == 0 {
			count += 1;
		}
	}
	println!("Final result: {count}");
}
