use std::fs;

use peggle::{Parse, ParseError};
use peggle_derive::Parse;


#[derive(Parse)]
#[peg("Game <game_number>: (<handfuls>; )*<handfuls>")]
struct GameRound {
    game_number: usize,
    handfuls: Vec<Handful>,
}

// Annoyed that this bit is rather verbose...
#[derive(Parse)]
#[peg("(<r>, <g>, <b>)|(<r>, <b>, <g>)|(<b>, <r>, <g>)|(<b>, <g>, <r>)|(<g>, <r>, <b>)|(<g>, <b>, <r>)|(<r>, <b>)|(<r>, <g>)|(<b>, <r>)|(<b>, <g>)|(<g>, <r>)|(<g>, <b>)|<r>|<b>|<g>")]
struct Handful {
    r: Option<Red>,
    g: Option<Green>,
    b: Option<Blue>,
}

#[derive(Parse)]
#[peg("<0> red")]
struct Red(usize);

#[derive(Parse)]
#[peg("<0> green")]
struct Green(usize);

#[derive(Parse)]
#[peg("<0> blue")]
struct Blue(usize);


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_correct = 0;

    for (idx, line) in input.lines().enumerate() {
        let Ok(round) = GameRound::parse(line) else {
            println!("detected invalid input");
            std::process::exit(1);
        };

        if is_possible_game(&round) {
            total_correct += round.game_number;
        }
    }

    println!("{}", total_correct);
}

fn is_possible_game(round: &GameRound) -> bool {
    for handful in &round.handfuls {
        if let Some(Red(13..)) = handful.r {
            return false
        }
        if let Some(Green(14..)) = handful.g {
            return false
        }
        if let Some(Blue(15..)) = handful.b {
            return false
        }
    }
    
    true
}
