use std::{cmp, fs};

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

    let mut sum_of_powers = 0usize;

    for (idx, line) in input.lines().enumerate() {
        let Ok(round) = GameRound::parse(line) else {
            println!("invalid input detected");
            std::process::exit(1);
        };

        let (min_red, min_green, min_blue) = min_cubes(&round);
        sum_of_powers += min_red * min_green * min_blue;
    }

    println!("{}", sum_of_powers);
}

fn min_cubes(round: &GameRound) -> (usize, usize, usize) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for handful in &round.handfuls {
        if let Some(Red(curr_red)) = handful.r {
            min_red = cmp::max(min_red, curr_red);
        }
        if let Some(Green(curr_green)) = handful.g {
            min_green = cmp::max(min_green, curr_green);
        }
        if let Some(Blue(curr_blue)) = handful.b {
            min_blue = cmp::max(min_blue, curr_blue);
        }
    }

    (min_red, min_green, min_blue)   
}
