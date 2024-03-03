use std::io::{self, Write};
use rand::Rng;

const MAX_NUMBER: u32 = 100;

#[derive(Debug)]
struct Player {
    name: String,
    number_of_tries: u8,
    current_guess:u32,
    guesses: Vec<u32>,
    guess_count: u8,
}

pub struct NumberGuessingGame;

impl NumberGuessingGame {

    fn create_randon_number(range: u32) -> u32 {

        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(1..=range);
        random_number

    }

    pub fn start_game() {

        print!("Enter Player Name: ");
        io::stdout().flush().expect("Cannot flush");
        let mut name: String = String::new();
        io::stdin().read_line(&mut name).expect("Cannot read line.");
        name = name.trim().to_string();

        let mut player = Player {
            name,
            number_of_tries: 3,
            current_guess: 0,
            guesses: Vec::new(),
            guess_count: 1,
        };

        println!("-------------------------------------------");
        println!("Welcome {}!", player.name);
        println!("-------------------------------------------");

        let number_to_guess: u32 = NumberGuessingGame::create_randon_number(MAX_NUMBER);
        let mut input: String = String::new();

        while player.number_of_tries > 0 {

            print!("Enter your {} guess: ", player.guess_count);
            io::stdout().flush().expect("Cannot flush");
            input.clear();
            io::stdin().read_line(&mut input).expect("Cannot read line.");
            player.current_guess = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a Valid Number.");
                    continue;
                }
            };
            
            player.guess_count += 1;
            player.guesses.push(player.current_guess);

            if player.current_guess == number_to_guess {
                println!("Congratulations {}! You Won!", player.name);
                return;
            } else {
                player.number_of_tries -= 1;
                println!("Wrong Guess.\nNumber of Attempts Left: {}", player.number_of_tries);
                println!("Your Guesses: {:?}", player.guesses);
                println!("-------------------------------------------");
            }
        }

        println!("You Lose {}!\nThe Number Was {}", player.name, number_to_guess);
    }
}