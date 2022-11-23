use std::io::Write;

use rockpaperscissors_rust::{Outcome, Choice};

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    };
}
fn main() {
    loop {
        // get user input
        let mut user_input = String::new();
        // send prompt to stdout
        print!("Choose rock, paper, or scissors (r, p, s):  ");
        // flush stdout
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut user_input).unwrap();
        // create choice, if invalid, loop again
        let user_choice = skip_fail!(Choice::new(user_input.trim().to_string()));

        // get random computer choice
        let possible_choices = ["rock", "paper", "scissors"];
        let computer_choice = Choice::new(possible_choices[rand::random::<usize>() % 3].to_string()).unwrap();

        // compare choices
        let outcome = user_choice.compare(&computer_choice);

        // print results
        println!("You chose: {}", user_choice.text);
        println!("Computer chose: {}", computer_choice.text);
        match outcome {
            Outcome::Win => println!("You win!"),
            Outcome::Lose => println!("You lose!"),
            Outcome::Draw => println!("It's a draw!"),
        }
    }
}
