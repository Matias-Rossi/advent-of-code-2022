use std;
use std::fs::File;
use std::collections::HashMap;
//use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors
}


fn main() {

    let plays = HashMap::from([
        ("A", Play::Rock),
        ("B", Play::Paper),
        ("C", Play::Scissors),
        ("X", Play::Rock),
        ("Y", Play::Paper),
        ("Z", Play::Scissors),
    ]);

    let mut player_points: u16 = 0;

    
    if let Ok(lines) = read_lines("./resources/input.txt") {
        

        for line in lines {
            if let Ok(ip) = line {
                let round_points = get_round_points_from_line(ip, plays.clone());
                player_points += round_points;
            }
        }
    }

    println!("Player points: {player_points}"); //13446
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_winner(play_1: Play, play_2: Play) -> u8 {
    //returns 1 if play_1 beats play_2, 2 if play_2 beats play_1, 0 in case of draw
    if play_1 == play_2 {
        return 0;
    }

    match play_1 {
        Play::Rock => {
            return if play_2 == Play::Scissors { 1 } else { 2 }
        }
        Play::Paper => { 
            return if play_2 == Play::Rock { 1 } else { 2 }
        }
        Play::Scissors => { 
            return if play_2 == Play::Paper { 1 } else { 2 }
        }
    }

}

fn get_play_points(play: Play) -> u16 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}

fn get_round_points_from_line(line: String, plays_map: HashMap<&str, Play>) -> u16 {
    let mut line_chars = line.chars(); 
    let opponent_char = line_chars.nth(0).unwrap().to_string();
    let player_char = line_chars.nth(1).unwrap().to_string();

    let opponent_play = plays_map[&opponent_char.as_str()];
    let player_play = plays_map[&player_char.as_str()];

    let winner_number = get_winner(opponent_play, player_play);
    let play_points = get_play_points(player_play);


    match winner_number {
        0 => {
            return play_points + 3;
        }
        1 => {
            return play_points;
        }
        2 => {
            return play_points + 6;
        }
        _ => {
            panic!("Match error")
        }
    }

}