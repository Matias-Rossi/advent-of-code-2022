use std::env;
use utils::input::read_lines;
use utils::matrix::matrix::Matrix;

#[derive(PartialEq, Eq, Debug)]
struct Rearrangement {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    if let Ok(lines) = read_lines("./resources/input.txt") {
        let mut crates_pos: Matrix<char> = Matrix { rows: Vec::new() };
        let mut initial_conditions_loaded = false;

        for line in lines {
            if let Ok(text) = line {
                let line_describes_crates_pos = text.starts_with("[");
                let line_describes_crate_mov = text.starts_with("move ");

                if line_describes_crates_pos {
                    //Handle initial data load
                    //crates_pos.rows.push(get_row_crates(&text));
                    crates_pos.rows.push(get_row_crates(&text));
                } else if line_describes_crate_mov {
                    //Handle crate movement
                    let r = get_movement_procedure(&text);
                    move_crates_between_rows(&mut crates_pos, r);
                } else if !initial_conditions_loaded {
                    //Process data after loading all initial rows were processed

                    crates_pos = crates_pos.rotate();
                    crates_pos.remove_nones();

                    initial_conditions_loaded = true;
                }
            }
        }

        print_last_value_from_rows(&mut crates_pos); //VRQWPDSGP
    }
}

fn get_row_crates(line: &String) -> Vec<Option<char>> {
    let mut crates: Vec<Option<char>> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 {
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
            3 => from = string.parse::<usize>().unwrap() - 1,
            5 => to = string.parse::<usize>().unwrap() - 1,
            _ => (),
        }
    }

    Rearrangement { amount, from, to }
}

fn move_crates_between_rows(matrix: &mut Matrix<char>, r: Rearrangement) {
    let mut crates_yet_to_be_moved = r.amount;

    while crates_yet_to_be_moved > 0 {
        let mut crate_stack: Vec<Option<char>> = Vec::new();

        while row_has_some(matrix, r.from) && crates_yet_to_be_moved > 0 {
            let crate_being_moved = matrix.pop_from_row(r.from).unwrap();
            crate_stack.insert(0, Some(crate_being_moved));

            crates_yet_to_be_moved -= 1;
        }

        for supply_crate in crate_stack.iter() {
            matrix.push_to_row(r.to, supply_crate.unwrap());
        }
    }
}

fn print_last_value_from_rows(matrix: &mut Matrix<char>) {
    /* This function renders the rows useless, as the values are removed from them */
    for i in 0..matrix.rows.len() {
        let e = matrix.pop_from_row(i);

        let txt = match e {
            Some(supply_crate) => supply_crate,
            None => '_',
        };
        print!("{}", txt);
    }
}

fn row_has_some<T>(matrix: &mut Matrix<T>, row_index: usize) -> bool {
    let row = matrix.rows.iter().nth(row_index).unwrap();
    row.iter().any(|e| e.is_some())
}
