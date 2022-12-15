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

    impl<T: Copy + std::fmt::Debug> Matrix<T> {
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
                let mut new_column: Vec<Option<T>> = Vec::new();
                for row in &(self.rows) {
                    new_column.insert(0, row[column_index as usize]);
                }
                new_rows.push(new_column);
            }

            return Matrix { rows: new_rows };
        }

        pub fn pop_from_row(&mut self, row_index: usize) -> Option<T> {
            let mut last_some_element_index: usize = 0;
            let row = self.rows.iter_mut().nth(row_index).unwrap();

            //println!("popping from row {}: {:?}",row_index, row);

            for (i, elem) in row.iter_mut().enumerate() {
                match elem {
                    Some(_) => last_some_element_index = i,
                    None => (), 
                }
            }

            //row.push(None);

            row.remove(last_some_element_index)
        }

        pub fn push_to_row(&mut self, row_index: usize, element: T) {
            let mut last_some_element_index: usize = 0;
            let row = self.rows.iter_mut().nth(row_index).unwrap();
            let mut none_found = false;

            for (i, elem) in row.iter_mut().enumerate() {
                match elem {
                    Some(_) => last_some_element_index = i,
                    None => none_found = true, 
                }
            }

            if none_found {
                row.insert(last_some_element_index + 1, Some(element));
                row.remove(row.len() - 1);
            } else {
                row.push(Some(element));
            }

            //self.rows.iter_mut().nth(row).unwrap().push(element);
        }

        pub fn remove_nones(&mut self) {
            for row in self.rows.iter_mut() {
                row.retain(|e| e.is_some());
            }
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn matrix_size() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(2)]), Vec::from([Some(1), Some(2), Some(3)])]),
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
                rows: Vec::from([Vec::from([Some(1), Some(2), Some(3)]), Vec::from([Some(4), Some(5), Some(6)])]),
            };
            let transposed_matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(4)]), Vec::from([Some(2), Some(5)]), Vec::from([Some(3), Some(6)])]),
            };

            assert_eq!(matrix.transpose(), transposed_matrix);
        }

        #[test]
        fn transpose_with_blanks() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(2), None]), Vec::from([Some(4), Some(5), Some(6)])]),
            };
            let transposed_matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(4)]), Vec::from([Some(2), Some(5)]), Vec::from([None, Some(6)])]),
            };

            assert_eq!(matrix.transpose(), transposed_matrix);
        }

        #[test]
        fn rotate() {
            let matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(2), Some(3)]), Vec::from([Some(4), Some(5), Some(6)])]),
            };
            let rotated_matrix = Matrix {
                rows: Vec::from([Vec::from([Some(4), Some(1)]), Vec::from([Some(5), Some(2)]), Vec::from([Some(6), Some(3)])]),
            };

            assert_eq!(matrix.rotate(), rotated_matrix);
        }

        #[test]
        fn pop_and_push_to_row() {
            let mut matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(2), Some(3)]), Vec::from([Some(4), Some(5), Some(6)])]),
            };
            let e = matrix.pop_from_row(1).unwrap();
            matrix.push_to_row(0, e);

            let expected_matrix = Matrix {
                rows: Vec::from([Vec::from([Some(1), Some(2), Some(3), Some(6)]), Vec::from([Some(4), Some(5)])]),
            };

            assert_eq!(matrix, expected_matrix);
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
