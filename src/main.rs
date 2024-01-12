use std::io::Read;
use std::fs::File;

const LENGTH_OF_MEMORY: usize = 30000;

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	let use_custom_instructions = args.contains(&String::from("--debug"));

	let mut source_file =
		File::options()
		.read(true)
		.open(args[1].clone())
		.expect("Failed to load source file");

	let mut source = String::new();
	source_file.read_to_string(&mut source).expect("Failed to read source file");

	let mut memory: [u8; LENGTH_OF_MEMORY] = [0; LENGTH_OF_MEMORY];
	let mut memory_index: usize = 0;
	let mut instruction_index: usize = 0;
	let mut loops_stack: Vec<usize> = vec![];

	while instruction_index < source.len() {
		let current_instruction =
			source
			.chars()
			.nth(instruction_index)
			.expect(&format!("Failed to read character at instruction index: {}", instruction_index));

		match (current_instruction, use_custom_instructions) {
			('<', _) => {
				if memory_index == 0 {
					memory_index = LENGTH_OF_MEMORY - 1;
				} else {
					memory_index -= 1;
				}
			}

			('>', _) => {
				if memory_index == LENGTH_OF_MEMORY - 1 {
					memory_index = 0;
				} else {
					memory_index += 1;
				}
			}

			('-', _) => memory[memory_index] -= 1,
			('+', _) => memory[memory_index] += 1,

			('.', _) => {
				if let Some(c) = char::from_u32(memory[memory_index] as u32) {
					print!("{}", c);
				}
			}

			(',', _) => {
				let input =
					std::io::stdin()
					.bytes()
					.next()
					// lmao
					.unwrap_or(Ok(0))
					.unwrap_or(0);

				memory[memory_index] = input;
			}

			('[', _) => {
				if memory[memory_index] != 0 {
					loops_stack.push(instruction_index);
				} else {
					while source.chars().nth(instruction_index).expect("Unbalanced brackets!") != ']' {
						instruction_index += 1;
					}
				}
			}

			(']', _) => {
				if memory[memory_index] != 0 {
					instruction_index = loops_stack[loops_stack.len() - 1];
				} else {
					loops_stack.pop().expect("Unbalanced brackets!");
				}
			}

			// Custom instructions
			(':', true) => {
				print!("{} ", memory[memory_index]);
			}

			('#', true) => {
				println!("\n{:?}", memory);
			}

			_ => {}
		}

		instruction_index += 1;
	}
}