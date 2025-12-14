use std::fs;

fn main() {
	let file_path = "./input.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let lines = contents.split('\n');
	let mut pointer_position = 50;
	let mut zero_hits = 0;
	for line in lines {
		let rotation_str: String = line.chars().skip(1).collect();
		let rotation = rotation_str.parse::<i32>().unwrap();
		if line.starts_with("R") {
			pointer_position += rotation;
		} else if line.starts_with("L") {
			pointer_position -= rotation;
		}
		// make sure pointer_position stays in bounds
		pointer_position %= 100;
		if pointer_position == 0 {
			zero_hits += 1;
		}
	}
	println!("Zero was hit {zero_hits} times");
}
