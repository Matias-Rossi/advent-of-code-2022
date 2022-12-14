use std;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet};



fn main() {

    // Open file
    if let Ok(lines) = read_lines("./resources/input.txt") {

        let mut repeated_items: Vec<char> = Vec::new();
        
        for line in lines {
            if let Ok(text) = line {
                let line_items = get_compartments_items_from_line(text);

                for item in line_items.iter() {
                    repeated_items.push(*item);
                }
            }

        }

        let mut priority_sum: u32 = 0;

        for c in repeated_items.iter() {
            priority_sum += get_priority_from_item(*c);
        }

        println!("\nPriority sum: {priority_sum}"); //7821


    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_compartments_items_from_line(line: String) -> HashSet<char> {
    let compartment_strs = line.split_at(line.len() / 2);

    let mut comp_1 = get_compartment(compartment_strs.0.to_string());
    let mut comp_2 = get_compartment(compartment_strs.1.to_string());

    return inplace_intersection(&mut comp_1, &mut comp_2);

    
}

fn get_compartment(compartment: String) -> HashSet<char> {
    let mut char_set = HashSet::new();
    for character in compartment.chars() {
        char_set.insert(character);
    }

    return char_set;
}

fn get_priority_from_item(character: char) -> u32 {
    let ascii_value = character as u32;

    //lowercase
    if ascii_value >= 97 {
        return ascii_value - 96;
    } else {
        //uppercase
        return ascii_value - 64 + 26;
    }
}

fn inplace_intersection(a: &mut HashSet<char>, b: &mut HashSet<char>) -> HashSet<char> {
    let mut c = HashSet::new();
    
    for v in a.iter() {
        if let Some(found) = b.take(v) {
            c.insert(found);
        }
    }
    
    a.retain(|v| !c.contains(&v));

    c
}