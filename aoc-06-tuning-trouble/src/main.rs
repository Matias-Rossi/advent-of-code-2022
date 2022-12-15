use utils::input::read_lines; //Single 4KB line
use std::{collections::HashSet, process::{self, exit}};

const DISTINCT_CHARACTERS: usize = 14; //4 for part A; 14 for part B

fn main() {
    if let Ok(lines) = read_lines("./resources/input.txt") {
        let mut chars_read: usize = 0;
        let mut last_chars_read: Vec<char> = Vec::new();

        for line in lines {
            if let Ok(text) = line {

                for c in text.chars() {
                    add_char_to_list(&mut last_chars_read, c);
                    let values_found = are_chars_different(&last_chars_read);

                    chars_read += 1;

                    if values_found {
                        //println!("Found first start-of-packet marker after processing {} characters.", chars_read); //1794
                        println!("Found first start-of-message marker after processing {} characters.", chars_read); //

                        exit(0);
                    }

                }
            }
        }
    }
}

fn add_char_to_list(list: &mut Vec<char>, character: char) -> Vec<char>{
    list.insert(0, character);

    if list.len() > DISTINCT_CHARACTERS {
        list.pop();
    }

    list.to_vec()
}

fn are_chars_different(list: &Vec<char>) -> bool {
    if list.len() < DISTINCT_CHARACTERS {
        return false;
    }

    let mut set = HashSet::new();

    for c in list.iter() {
        set.insert(c);
    }

    set.len() == DISTINCT_CHARACTERS
}