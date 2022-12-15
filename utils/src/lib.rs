pub mod matrix;



pub mod set {
    use std::collections::HashSet;

    pub fn set_intersection(a: &mut HashSet<char>, b: &mut HashSet<char>) -> HashSet<char> {
        let mut c = HashSet::new();

        for v in a.iter() {
            if let Some(found) = b.take(v) {
                c.insert(found);
            }
        }

        a.retain(|v| !c.contains(&v));

        c
    }

    #[cfg(test)]
    mod tests {
        use super::set_intersection;
        use super::*;

        #[test]
        fn void_intersection() {
            let mut a = HashSet::from(['a', 'b']);
            let mut b = HashSet::from(['c', 'd']);

            let result = set_intersection(&mut a, &mut b);
            assert!(result.is_empty());
        }

        #[test]
        fn interesction() {
            let mut a = HashSet::from(['a', 'b']);
            let mut b = HashSet::from(['b', 'd']);

            let result = set_intersection(&mut a, &mut b);
            assert_eq!(result, HashSet::from(['b']));
        }
    }
}

pub mod input {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
