use std::io;
 
fn main() {
    println!("Hello, Lets play Rock, Paper, Scissors!");
 
    println!("Please select (r)ock, (p)aper, or (s)cissors:");
 
    let mut player_move = String::new();
 
    io::stdin()
    
    	.read_line(&mut player_move)
        
        .expect("Failed to read move");
        
    println!("You guessed: {player_move}");
}
enum RockPaperScissorsGuess { 
    Rock, 
    Paper,
    Scissors,
    let comp_move = rand::thread_rng().gen_range(1..=3);
}