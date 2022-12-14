use utils::input::read_lines;
use utils::matrix::{Matrix};
use std::env;

struct Rearrangement {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    if let Ok(lines) = read_lines("./resources/input.txt") {

        let mut crates_pos: Matrix<Option<char>> = Matrix{ rows: Vec::new()};
        let mut initial_conditions_loaded = false;

        for line in lines {
            if let Ok(text) = line {
                let line_describes_crates_pos = text.starts_with("[");
                let line_describes_crate_mov = text.starts_with("move ");

                if line_describes_crates_pos {
                    //Handle initial data load
                    crates_pos.rows.push(get_row_crates(&text));
                    
                } else
                if line_describes_crate_mov {
                    //Handle crate movement
                    let r = get_movement_procedure(&text);
                    move_crates_between_rows(&mut crates_pos, r);


                } else
                if !initial_conditions_loaded {
                    //Process data after loading all initial rows were processed
                    
                    crates_pos = crates_pos.rotate();


                    initial_conditions_loaded = true;

                }
            }
        }

        print_last_value_from_rows(&mut crates_pos);

    }
}

fn get_row_crates(line: &String) -> Vec<Option<char>> {
    let mut crates: Vec<Option<char>> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if i % 4 == 2 {
            if c != ' ' {
                crates.push(Some(c));
            } else {
                crates.push(None);
            }
        }
    }

    crates
}

fn get_movement_procedure(line: &String) -> Rearrangement {
    let mut amount = 0;
    let mut from = 0;
    let mut to = 0;
    
    for (i, string) in line.split(" ").enumerate() {
        
        match i {
            1 => amount = string.parse::<usize>().unwrap(),
            3 => from = string.parse::<usize>().unwrap(),
            5 => to = string.parse::<usize>().unwrap(),
            _ => (),
        }
    }

    Rearrangement { amount, from, to }
    
}


fn move_crates_between_rows(matrix: &mut Matrix<Option<char>>, r: Rearrangement) {
    for _ in 0..(r.amount) {
        let crate_being_moved = matrix.pop_from_row(r.from).unwrap();
        matrix.push_to_row(r.to, crate_being_moved);
    }
}

fn print_last_value_from_rows(matrix: &mut Matrix<Option<char>>) {
    println!("Last elements from each crate_column are: ");
    for row in matrix.rows.iter() {
        print!("{}, ", row.last().unwrap().unwrap());
    }
}