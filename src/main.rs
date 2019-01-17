extern crate rand;
#[macro_use]
extern crate text_io;

use rand::prelude::*;

fn main() {
    println!("Welcome to The Game");
    println!("What is your name player?");
    let player_name: String = read!("{}\n");

    println!("Ok, {}! The rules are simple.", player_name);
    println!("Guess a number between 0 and 100.");
    println!("If you guess correctly you win!");

    println!("Are you ready? (Y/N)");
    let pregame_answer: String = read!("{}\n");

    if pregame_answer == "Y" || pregame_answer == "y" {
        loop {
            let mut rng = thread_rng();
            let random_num = rng.gen_range(0, 101);
            println!("So go on... Guess");

            loop {

                let player_num: i32 = read!("{}\n");

                if player_num > random_num {
                    println!("The number I was thinking of is a lil lower");
                } else if player_num < random_num {
                    println!("The number I was thinking of is a lil higher");
                } else {
                    println!("Well done you got it right, the random number is: {}", random_num);
                    break;
                }
            }

            println!("Wanna play another round? (Y/N)");
            let player_choice: String = read!("{}\n");

            if player_choice == "Y" || player_choice == "y" {
                println!("Yeay!");
            } else {
                break;
            }
        }
    }
    println!("I'm sad to see you leave, bye");
}
