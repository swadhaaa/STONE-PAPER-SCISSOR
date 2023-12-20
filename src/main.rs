extern crate rand;
use std::io;
use rand::Rng;

fn main() {
	let mut _rng = rand::thread_rng();
	println!("1 - Rock, 2 - Paper, 3 - Scissors");

	let mut players_choice = String::new();
	let mut players_choice_int = 0;
	
	io::stdin().read_line(&mut players_choice).expect("Cannot read the players_choice");
	
	let comp_choice = rand::thread_rng().gen_range(1..=3);
	
	match comp_choice {
		1 => println!("Computer chose Rock"),
		2 => println!("Computer chose Paper"),
		3 => println!("Computer chose Scissors"),
		_ => println!("Error!"),
	}
	
	if players_choice == "1\n" {
		players_choice_int = 1;
    	println!("You chose Rock");
	} else if players_choice == "2\n" {
		players_choice_int = 2;
    	println!("You chose Paper");
	} else if players_choice == "3\n"{
		players_choice_int = 3;
    	println!("You chose Scissors");
	} else {
		println!("Error!");
	}
	let compare = (players_choice_int, comp_choice);
	

	match compare {
		(1, 1) => println!("It's a tie!"),
		(1, 2) => println!("You lose!"),
		(1, 3) => println!("You win!"),
		(2, 1) => println!("You win!"),
		(2, 2) => println!("It's a tie!"),
		(2, 3) => println!("You lose!"),
		(3, 1) => println!("You lose!"),
		(3, 2) => println!("You win!"),
		(3, 3) => println!("It's a tie!"),
		_ => println!("Error!"),
	}
}