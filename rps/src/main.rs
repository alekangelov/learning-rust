use rand::Rng;
use std::io;
use std::io::Error;
use std::str::FromStr;
#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Score {
    player1: u32,
    player2: u32,
}

impl Score {
    fn new() -> Score {
        Score {
            player1: 0,
            player2: 0,
        }
    }

    fn add(&mut self, player: u8) {
        match player {
            0 => (),
            1 => self.player1 += 1,
            2 => self.player2 += 1,
            _ => (),
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" => Ok(Hand::Rock),
            "p" => Ok(Hand::Paper),
            "s" => Ok(Hand::Scissors),
            _ => Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid input. Please try again.",
            )),
        }
    }
}

fn get_hand() -> Hand {
    let mut input = String::new();
    println!("Choose your hand: (Rock (r), Paper (p), Scissors (s)) \n");
    io::stdin().read_line(&mut input).unwrap();
    match Hand::from_str(input.trim()) {
        Ok(hand) => hand,
        Err(_) => {
            println!("Invalid input. Please try again.");
            get_hand()
        }
    }
}

fn gen_hand() -> Hand {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..=2);
    match num {
        0 => Hand::Rock,
        1 => Hand::Paper,
        2 => Hand::Scissors,
        _ => Hand::Rock,
    }
}

fn compare_hands() -> u8 {
    let player1 = get_hand();
    let player2 = gen_hand();
    println!("Player: {:?}", player1);
    println!("Computer: {:?}", player2);
    match (player1, player2) {
        (Hand::Rock, Hand::Rock) => {
            println!("Draw!\n");
            0
        }
        (Hand::Rock, Hand::Paper) => {
            println!("Player 2 wins!\n");
            2
        }
        (Hand::Rock, Hand::Scissors) => {
            println!("Player 1 wins!\n");
            1
        }
        (Hand::Paper, Hand::Rock) => {
            println!("Player 1 wins!\n");
            1
        }
        (Hand::Paper, Hand::Paper) => {
            println!("Draw!\n");
            0
        }
        (Hand::Paper, Hand::Scissors) => {
            println!("Player 2 wins!\n");
            2
        }
        (Hand::Scissors, Hand::Rock) => {
            println!("Player 2 wins!\n");
            2
        }
        (Hand::Scissors, Hand::Paper) => {
            println!("Player 1 wins!\n");
            1
        }
        (Hand::Scissors, Hand::Scissors) => {
            println!("Draw!\n");
            0
        }
    }
}

fn continue_game() -> bool {
    println!("Do you want to play again? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

fn main() {
    let mut score = Score::new();
    loop {
        println!("Rock, Paper, Scissors!");
        let who_won = compare_hands();
        score.add(who_won);
        println!("Player 1: {:?}", score.player1);
        println!("Player 2: {:?}", score.player2);
        if !continue_game() {
            break;
        }
    }
}
