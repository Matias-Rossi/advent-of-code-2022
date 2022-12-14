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

pub mod matrix {

    #[derive(PartialEq, Debug)]
    pub struct Matrix<T> {
        pub rows: Vec<Vec<Option<T>>>,
    }

    impl<T: Copy> Matrix<T> {
        pub fn size(&self) -> (u32, u32) {
            let height = self.rows.len() as u32;

            let mut max_width: u32 = 0;

            for row in self.rows.iter() {
                let rows = row.len() as u32;
                if rows > max_width {
                    max_width = rows;
                }
            }

            (max_width, height)
        }

        pub fn transpose(&self) -> Self {
            let (original_w, _) = self.size();

            let mut new_rows = Vec::new();

            for column_index in 0..original_w {
                let mut new_column = Vec::new();

                for row in &(self.rows) {
                    new_column.push(row[column_index as usize]);
                }
                new_rows.push(new_column);
            }

            return Matrix { rows: new_rows };
        }

        pub fn rotate(&self) -> Self {
            let (original_w, _original_h) = self.size();

            let mut new_rows = Vec::new();

            for column_index in 0..original_w {
                let mut new_column: Vec<T> = Vec::new();
                for row in &(self.rows) {
                    new_column.insert(0, row[column_index as usize]);
                }
                new_rows.push(new_column);
            }

            return Matrix { rows: new_rows };
        }

        pub fn pop_from_row(&mut self, row: usize) -> Option<T> {
            self.rows.iter_mut().nth(row).unwrap().pop()
        }

        pub fn push_to_row(&mut self, row: usize, element: T) {
            self.rows.iter_mut().nth(row).unwrap().push(element);
        }

        /*
        fn set_value(&mut self, new_value: T, (column, row): (u32, u32)) {

            let (width, height) = self.size();
            println!("Position ({column}, {row}) to be set at matrix with size ({height}, {width})");

            let row = self.rows.iter_mut().nth(row as usize).unwrap();
            let value_to_be_changed = &mut *(row.iter_mut().nth(column as usize).unwrap());
            *value_to_be_changed = new_value;
        }

        fn get_value(&self, (column, row): (u32, u32)) -> &T {
            let row = self.rows.iter().nth(row as usize).unwrap();
            row.iter().nth(column as usize).unwrap()
        }
        */
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn matrix_size() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([1, 2]), Vec::from([1, 2, 3])]),
            };
            assert_eq!(matrix.size(), (3, 2));
        }

        #[test]
        fn empty_matrix_size() {
            let matrix: Matrix<bool> = Matrix {
                rows: Vec::from([]),
            };
            assert_eq!(matrix.size(), (0, 0));
        }

        #[test]
        fn transpose() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6])]),
            };
            let transposed_matrix = Matrix {
                rows: Vec::from([Vec::from([1, 4]), Vec::from([2, 5]), Vec::from([3, 6])]),
            };

            assert_eq!(matrix.transpose(), transposed_matrix);
        }

        #[test]
        fn rotate() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6])]),
            };
            let rotated_matrix = Matrix {
                rows: Vec::from([Vec::from([4, 1]), Vec::from([5, 2]), Vec::from([6, 3])]),
            };

            assert_eq!(matrix.rotate(), rotated_matrix);
        }

        /*
        #[test]
        fn get_set_value() {
            let mut matrix = Matrix {
                /*
                1 2 0
                1 2 3
                */
                rows: Vec::from([Vec::from([1, 2, 0]), Vec::from([1, 2, 3])]),
            };

            let pos: (u32, u32) = (1, 2);

            matrix.set_value(8, pos);
            matrix.get_value(pos);

            /*
                1 2 0
                1 2 8
            */

            assert_eq!(matrix.get_value(pos), &8);
        }
         */
    }
}
