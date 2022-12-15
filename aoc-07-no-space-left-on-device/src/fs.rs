use std::boxed::Box;

pub mod filesystem {
    use std::collections::{HashMap, HashSet};

    trait Sizeable {
        fn get_size(&self) -> usize;
    }

    /* DIRECTORIES */

    #[derive(Debug)]
    pub struct Dir {
        pub files: HashMap<String, File>,
        pub subdirectories: HashMap<String, Dir>,
        size: usize,
    }

    pub fn create_dir(files: HashMap<String, File>, subdirectories: HashMap<String, Dir>) -> Dir {
        let mut dir = Dir {
            files,
            subdirectories,
            size: 0,
        };
        dir.calculate_size();

        dir
    }

    impl Sizeable for Dir {
        fn get_size(&self) -> usize {
            let mut size_sum: usize = 0;

            for (_, file) in self.files.iter() {
                size_sum += file.get_size();
            }
            for (_, dir) in self.subdirectories.iter() {
                size_sum += dir.get_size();
            }

            size_sum
        }
    }

    impl Dir {
        pub fn calculate_size(&mut self) {
            self.size = self.get_size();
        }

        pub fn get_or_create_subdirectory(&mut self, name: &str) -> &Dir {
            if !self.subdirectories.contains_key(name) {
                self.subdirectories.insert(
                    (*name).to_string(),
                    Dir {
                        files: HashMap::new(),
                        size: 0,
                        subdirectories: HashMap::new(),
                    },
                );
            }
            self.subdirectories.get(name).unwrap()
        }
    }

    /* FILES */

    #[derive(Debug)]
    pub struct File {
        name: String,
        size: usize,
    }

    impl Sizeable for File {
        fn get_size(&self) -> usize {
            self.size
        }
    }
}
