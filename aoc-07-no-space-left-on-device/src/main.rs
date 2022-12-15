use core::panic;
use std::{collections::{HashSet, HashMap}, thread::current};

use fs::filesystem::{Dir, create_dir};
use utils::input::read_lines;
use std::boxed::Box;


pub mod fs;

enum Command {
    Ls,
    Cd,
}


fn main() {
    if let Ok(lines) = read_lines("./resources/input.txt") {
        let mut root_directory = create_dir(HashMap::new(), HashMap::new());
        let mut path: Vec<&Dir> = Vec::new();
        let mut current_directory: &Dir = &root_directory;
        let mut last_command: Command = Command::Cd;


        for line in lines {
            if let Ok(text) = line {
                let mut tokens: Vec<&str> = text.split(" ").collect();

                if is_command(tokens.iter().nth(0).unwrap()) {
                    //Handle user input
                    
                    last_command = handle_user_input(tokens, path);

                } else {
                    //Read data

                }
            }
        }
    }
}

fn is_command(text: &&str) -> bool {
    text.starts_with("$")
}

fn handle_user_input<'a>(tokens: Vec<&str>, mut path: Vec<&'a Dir>) -> Command {
    let command: Command;
    match tokens.first().unwrap() {
        &"ls" => {
            command = Command::Ls
        },
        &"cd" => {
            command = Command::Cd;
            let cd_target = tokens.iter().nth(1).unwrap();
    
            if cd_target == &".." {
                go_up_one_level(&mut path);
            }
            else if cd_target == &"/" {
                go_to_root(&mut path);
            }
            else {
                //Specific subdirectory
                go_to_subdir(&mut path, cd_target.to_string());
            }
            
        },
        _ => panic!("unexpected token"),

    }
    return command;
}


fn get_current_dir<'a>(path: &'a Vec<&'a Dir>) -> &'a Dir {
    path.last().unwrap()
}

fn get_current_dir_mut(path: Vec<Dir>) -> &mut Dir {
    &(path.last().unwrap())
}

fn go_to_root(path: &mut Vec<&Dir>) {
    path.truncate(1);
}

fn go_up_one_level<'a>(path: &'a mut Vec<&'a Dir>) {
    path.truncate(path.len()-1);
}

fn go_to_subdir<'a>(path: &'a mut Vec<&'a Dir>, subdir_name: String)  {
    let current_dir = get_current_dir(&path);
    let new_dir: &mut Dir = &mut *current_dir.get_or_create_subdirectory(&subdir_name);
    path.push(&new_dir);
}