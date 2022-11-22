use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Guess a number:");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		println!("You guessed: {guess}");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please type a number!");
				continue;
			}
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		};
	}
}
