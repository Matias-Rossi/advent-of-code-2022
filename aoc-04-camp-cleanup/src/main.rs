use utils::input::read_lines;
use std::collections::HashSet;

fn main() {

    let mut subset_assignments_found: u32 = 0;

    if let Ok(lines) = read_lines("./resources/input.txt") {
        for line in lines {
            if let Ok(text) = line {

                if check_line_for_subset(text) {
                    subset_assignments_found += 1;
                }

            }
        }
    }   

    println!("{subset_assignments_found} assignment pairs in which one range fully contains the other were found."); //595
}

fn get_elf_sections(text: &str) -> HashSet<u16> {
    let (lower_limit, upper_limit) = text.split_once('-').unwrap();
    let (lower_limit, upper_limit): (u16, u16) = (lower_limit.parse::<u16>().unwrap(), upper_limit.parse::<u16>().unwrap() + 1);
    
    let mut sections: HashSet<u16> = HashSet::new();

    for n in lower_limit..upper_limit {
        sections.insert(n);
    }
    
    sections

}

fn check_line_for_subset(line: String) -> bool {
    let elves_ranges: Vec<&str> = line.split(',').collect();
    let mut elves_sections: Vec<HashSet<u16>> = elves_ranges.iter().map(|r| get_elf_sections(r)).collect();
    let set_a = elves_sections.pop().unwrap();
    let set_b = elves_sections.pop().unwrap();

    set_a.is_subset(&set_b) || set_b.is_subset(&set_a)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::get_elf_sections;

    #[test]
    fn elf_sections() {
        let result = get_elf_sections("5-10");
        assert_eq!(result, HashSet::<u16>::from([5,6,7,8,9,10]));
    }
}