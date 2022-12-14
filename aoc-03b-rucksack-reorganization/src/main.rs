/*
    https://stackoverflow.com/questions/64612490/how-to-solve-e0382-use-of-moved-value-in-a-for-loop

    Advice to Rust beginners: clone, clone shamelessly. Get comfortable with the language, 
    then take a big breath and dive into borrowing. Learning ownership/borrowing is hard 
    enough that you don't want to also stumble on other issues at the same time.
        - Matthieu M.

*/

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{self};

fn main() {
    // Open file
    if let Ok(lines) = read_lines("./resources/input.txt") {
        //Part 2 variables
        let mut possible_badges: HashSet<char> = HashSet::new();
        let mut processed_rucksacks: u16 = 0;
        let mut badges: Vec<char> = Vec::new();

        for line in lines {
            if let Ok(text) = line {
                match processed_rucksacks % 3 {
                    0 => possible_badges = get_rucksack_items(text),
                    2 => {
                        let mut rucksack_items = get_rucksack_items(text);
                        possible_badges = inplace_intersection(&mut possible_badges, &mut rucksack_items);                        
                        badges.push(get_badge_from_list(&possible_badges).unwrap())
                        
                    },
                    _ => {
                        let mut rucksack_items = get_rucksack_items(text);
                        {
                            possible_badges = inplace_intersection(&mut possible_badges, &mut rucksack_items);
                        }
                    },
                }
                /* 
                //On first rucksack of group
                if processed_rucksacks % 3 == 0 {
                    possible_badges = get_rucksack_items(text);
                }
                //On last group ruckasck
                else if processed_rucksacks % 3 == 2 {
                    badges.push(get_badge_from_list(possible_badges).unwrap());
                } else {
                    let mut rucksack_items = get_rucksack_items(text);
                    possible_badges =
                        inplace_intersection(&mut possible_badges, &mut rucksack_items);
                }
                */
                processed_rucksacks += 1;
            }
        }

        let mut priority_sum: u32 = 0;

        for c in &badges {
            priority_sum += get_priority_from_item(*c);
        }

        println!("\nPriority sum: {priority_sum}"); //7821

        println!("\nBadges found: ");

        for badge in badges {
            print!("{badge}, ");
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_rucksack_items(rucksack: String) -> HashSet<char> {
    let mut char_set = HashSet::new();
    for character in rucksack.chars() {
        char_set.insert(character);
    }

    return char_set;
}

fn get_badge_from_list(badge_set: &HashSet<char>) -> Option<char> {
    if badge_set.len() > 1 {
        panic!("More than one badge for a group were found");
    } else {
        for badge in badge_set.iter() {
            return std::option::Option::Some(*badge);
        }
        return None;
    }
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
