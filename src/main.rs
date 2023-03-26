use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(PartialEq)]
enum PlayResult {
    WIN,
    LOSE,
    DRAW
}

#[derive(PartialEq)]
enum Sign {
    ROCK,
    PAPER,
    SCISSORS,
    LIZARD,
    SPOCK
}

impl Sign {
    fn compare_signs(&self, other: &Sign) -> PlayResult {
        if *self == *other {
            PlayResult::DRAW
        } else if
        (*self == Sign::SCISSORS && *other == Sign::PAPER)
            || (*self == Sign::PAPER && *other == Sign::ROCK)
            || (*self == Sign::ROCK && *other == Sign::LIZARD)
            || (*self == Sign::LIZARD && *other == Sign::SPOCK)
            || (*self == Sign::SPOCK && *other == Sign::SCISSORS)
            || (*self == Sign::SCISSORS && *other == Sign::LIZARD)
            || (*self == Sign::LIZARD && *other == Sign::PAPER)
            || (*self == Sign::PAPER && *other == Sign::SPOCK)
            || (*self == Sign::SPOCK && *other == Sign::ROCK)
            || (*self == Sign::ROCK && *other == Sign::SCISSORS)
        {
            PlayResult::WIN
        } else {
            PlayResult::LOSE
        }
    }
}

struct Player {
    num: i32,
    sign: Sign,
    prev_players: Vec<i32>
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut players = Vec::<Player>::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let numplayer = parse_input!(inputs[0], i32);
        let signplayer = inputs[1].trim().to_string();

        let sign = match signplayer.as_str() {
            "R" => {
                Sign::ROCK
            }
            "P" => {
                Sign::PAPER
            }
            "C" => {
                Sign::SCISSORS
            }
            "L" => {
                Sign::LIZARD
            }
            _ => {
                Sign::SPOCK
            }
        };

        players.push(
            Player{
                num: numplayer,
                sign,
                prev_players: Vec::new()
            }
        );
    }


    while players.len() > 1 {
        players = play_next_round(players);
    }

    println!("{}", players[0].num);

    let mut output_str = String::new();
    for (i, v) in players[0].prev_players.iter().enumerate() {
        if i != 0 {
            output_str.push(' ');
        }
        output_str.push_str(v.to_string().as_str());
    }
    println!("{output_str}");

}

fn play_next_round(mut players: Vec::<Player>) -> Vec::<Player> {
    let mut output_vector = Vec::<Player>::new();
    while !players.is_empty() {
        let mut second = players.pop().expect("second player pop failed");
        let mut first = players.pop().expect("first player pop failed");

        let mut result = first.sign.compare_signs(&second.sign);

        if result == PlayResult::DRAW {
            result =
                if first.num < second.num {
                    PlayResult::WIN
                } else {
                    PlayResult::LOSE
                }
        }

        match result {
            PlayResult::WIN => {
                first.prev_players.push(second.num);
                output_vector.push(first);
            }
            PlayResult::LOSE => {
                second.prev_players.push(first.num);
                output_vector.push(second);
            }
            PlayResult::DRAW => {

            }
        }
    }

    output_vector
}