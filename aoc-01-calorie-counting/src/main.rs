use std;
use std::fs::File;
//use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

struct Elf {
    snacks: Vec<u32>,
    total_calories: u32
}

impl Default for Elf {
    fn default() -> Elf {
        Elf {
            snacks: Vec::new(),
            total_calories: 0
        }
    }
}


fn main() {

    // Open file
    if let Ok(lines) = read_lines("./resources/input.txt") {
        let mut elf = Elf { snacks: Vec::new(), total_calories: 0 };
        let mut elves: Vec<Elf> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                if ip.eq("") {
                    println!("Loaded elf with {} calories", elf.total_calories);
                    elves.push(elf);
                    elf = Elf { snacks: Vec::new(), total_calories: 0 };
                } else {
                    
                    let elf_calories: u32 = ip.parse::<u32>().unwrap();
                    elf.snacks.push(elf_calories);
                    elf.total_calories += elf_calories;
                }
                
                //println!("{}", ip);
            }
        }

        let mut max_calories = (0, 0, 0);

        for e in elves {
            if e.total_calories >= max_calories.0 {
                max_calories.2 = max_calories.1;
                max_calories.1 = max_calories.0;
                max_calories.0 = e.total_calories;
            }
            else 
            if e.total_calories >= max_calories.1 {
                max_calories.2 = max_calories.1;
                max_calories.1 = e.total_calories;
            } else 
            if e.total_calories >= max_calories.2 {
                max_calories.2 = e.total_calories;
            }
        }

        println!("Elf with most calories have the following calories");
        println!("1. {}", max_calories.0);
        println!("2. {}", max_calories.1);
        println!("3. {}", max_calories.2);
        println!("Total: {}", max_calories.0 + max_calories.1 + max_calories.2);

        //206519

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}