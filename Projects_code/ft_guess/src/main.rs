use std::io;
use rand::RngExt;

fn main() {
	println!("Wich value should be the maximum ?");
	let mut treshold1 = String::new();
	io::stdin()
        .read_line(&mut treshold1)
        .expect("Failed to read line");
	let treshold = treshold1.trim().parse::<i32>().unwrap();
    println!("I'm thinking of a number between 1 and {}", treshold);
	let res: i32 = rand::rng().random_range(1..treshold);
	let mut flag: i32 = 0;
	let mut i: i32 = 0;
	while flag == 0{
		println!("Guess: ");
		let mut guess1 = String::new();
		io::stdin()
			.read_line(&mut guess1)
			.expect("Failed to read line");
		let guess = guess1.trim().parse::<i32>().unwrap();
		i += 1;
		if guess == res{
			println!("You guessed it in {} try", i);
			flag = 1
		} else if guess > res {
			println!("No, {} is too high", guess);
		} else if 0 < guess && guess < res {
			println!("No, {} is too low", guess);
		} else if guess <= 0 {
			println!("Your guess must be at least greater than 0");
		} else {
			println!("You need to give a positive integer")
		}
	}
}
