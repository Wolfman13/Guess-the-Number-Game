extern crate rand;
#[macro_use]
extern crate text_io;
extern crate termion;

use rand::prelude::*;
use std::io;
use std::cmp::Ordering;
use termion::{color, clear};

//reset = color::Fg(color::Reset)

fn main() {
    println!("{}", clear::All);

    println!("{green}Welcome to The Game
        \nWhat is your name player?",
        green = color::Fg(color::Green));
    let player_name: String = read!("{}\n");

    println!("Ok, {red}{name}{green}! The rules are simple.
        \nGuess a number between 0 and 100.
        \nIf you guess correctly you win!",
        red = color::Fg(color::Red),
        green = color::Fg(color::Green),
        name = player_name);

    loop {
        println!("Are you ready? (Y/N)");
        let pregame_answer_lowercase: String = read!("{}\n");
        let pregame_answer = pregame_answer_lowercase.to_uppercase();


        if pregame_answer == "Y" {
            loop {
                let mut rng = thread_rng();
                let random_num = rng.gen_range(0, 101);
                println!("So go on... Guess");

                loop {
                    let mut guess = String::new();
                    io::stdin().read_line(&mut guess)
                        .expect("Failed to read line");
                    let guess: u32 = match guess.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("{blue}Please use only numbers{green}",
                                blue = color::Fg(color::Blue),
                                green = color::Fg(color::Green));
                            continue;
                        },
                    };

                    match guess.cmp(&random_num) {
                        Ordering::Less => println!("The number I was thinking of is a lil higher"),
                        Ordering::Greater => println!("The number I was thinking of is a lil lower"),
                        Ordering::Equal => {
                            println!("{blue}Well done you got it right!{green}",
                                blue = color::Fg(color::Blue),
                                green = color::Fg(color::Green));
                            break;
                        }
                    }
                }

                break;
            }
        } else if pregame_answer == "N" {
            println!("Maybe next time then");
            break;
        } else {
            println!("{blue}Please enter either 'Y' for yes or 'N' for no{green}",
                blue = color::Fg(color::Blue),
                green = color::Fg(color::Green));
        }
    }

    println!("{reset}{clear}",
        reset = color::Fg(color::Reset),
        clear = clear::All);
}
