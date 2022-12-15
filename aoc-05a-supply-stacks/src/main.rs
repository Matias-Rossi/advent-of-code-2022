use utils::input::read_lines;
use utils::matrix::matrix::Matrix;
use std::env;

#[derive(PartialEq, Eq, Debug)]
struct Rearrangement {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    if let Ok(lines) = read_lines("./resources/input.txt") {

        let mut crates_pos: Matrix<char> = Matrix{ rows: Vec::new()};
        let mut initial_conditions_loaded = false;

        for line in lines {
            if let Ok(text) = line {
                let line_describes_crates_pos = text.starts_with("[");
                let line_describes_crate_mov = text.starts_with("move ");

                if line_describes_crates_pos {
                    //Handle initial data load
                    //crates_pos.rows.push(get_row_crates(&text));
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
                    crates_pos.remove_nones();

                    initial_conditions_loaded = true;

                }
            }
        }

        print_last_value_from_rows(&mut crates_pos); //GFTNRBZPF

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
    for _ in 0..(r.amount) {
        let crate_being_moved = matrix.pop_from_row(r.from).unwrap();
        matrix.push_to_row(r.to, crate_being_moved);
    }
}

fn print_last_value_from_rows(matrix: &mut Matrix<char>) {
    /* This function renders the rows useless, as the values are removed from them */
    for i in 0..matrix.rows.len() {
        let e = matrix.pop_from_row(i);
        print!("{}", e.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_procedure() {
        let s: String = String::from("move 3 from 1 to 5");
        let r = get_movement_procedure(&s);

        assert_eq!(r, Rearrangement { amount: 3, from: 0, to: 4});
    }

    #[test]
    fn move_crates() {
        let r = Rearrangement { amount: 3, from: 0, to: 1};
        let mut matrix = Matrix {
            rows: Vec::from([Vec::from([Some('1'), Some('2'), Some('3')]), Vec::from([])]),
        };

        move_crates_between_rows(&mut matrix, r);

        let target_matrix = Matrix {
            rows: Vec::from([Vec::from([]), Vec::from([Some('3'), Some('2'), Some('1')])]),
        };

        assert_eq!(matrix, target_matrix);

    }

}