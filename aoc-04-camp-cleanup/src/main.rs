use std::collections::HashSet;
use utils::input::read_lines;

struct AssignmentPairStatus {
    contained_within: bool,
    overlap: bool,
}

fn main() {
    let mut subset_assignments_found: u32 = 0;
    let mut overlaps_found: u32 = 0;

    if let Ok(lines) = read_lines("./resources/input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let assignment_pair_status = check_line_for_subset_and_overlap(text);
                if assignment_pair_status.contained_within {
                    subset_assignments_found += 1;
                    overlaps_found += 1;
                } else if assignment_pair_status.overlap {
                    overlaps_found += 1;
                }
            }
        }
    }

    println!("{subset_assignments_found} assignment pairs in which one range fully contains the other were found.");
    //595
    println!("{overlaps_found} assiognment pairs overlap.");
}

fn get_elf_sections(text: &str) -> HashSet<u16> {
    let (lower_limit, upper_limit) = text.split_once('-').unwrap();
    let (lower_limit, upper_limit): (u16, u16) = (
        lower_limit.parse::<u16>().unwrap(),
        upper_limit.parse::<u16>().unwrap() + 1,
    );

    let mut sections: HashSet<u16> = HashSet::new();

    for n in lower_limit..upper_limit {
        sections.insert(n);
    }

    sections
}

fn check_line_for_subset_and_overlap(line: String) -> AssignmentPairStatus {
    let elves_ranges: Vec<&str> = line.split(',').collect();
    let mut elves_sections: Vec<HashSet<u16>> =
        elves_ranges.iter().map(|r| get_elf_sections(r)).collect();
    let set_a = elves_sections.pop().unwrap();
    let set_b = elves_sections.pop().unwrap();

    let one_is_subset_from_other = set_a.is_subset(&set_b) || set_b.is_subset(&set_a);
    let overlap = if one_is_subset_from_other {
        true
    } else {
        set_a.intersection(&set_b).count() > 0
    };

    AssignmentPairStatus {
        contained_within: one_is_subset_from_other,
        overlap,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::get_elf_sections;

    #[test]
    fn elf_sections() {
        let result = get_elf_sections("5-10");
        assert_eq!(result, HashSet::<u16>::from([5, 6, 7, 8, 9, 10]));
    }
}
