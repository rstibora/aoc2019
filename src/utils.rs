pub mod file_handling {
    use std::{io, fs, path::PathBuf};

    pub fn get_input_filename(day_number: u32, filename_suffix: Option<&str>) -> String {
        let suffix = filename_suffix.unwrap_or("").to_owned();
        format!("day_{:0>2}{}", day_number, suffix)
    }

    pub fn get_input_extension() -> String {
        String::from("input")
    }

    pub fn get_input_for_day(day_number: u32, filename_suffix: Option<&str>, folder: &str) -> Result<String, io::Error> {
        let path = get_whole_file_path(folder, &get_input_filename(day_number, filename_suffix), &get_input_extension());
        fs::read_to_string(path)
    }

    fn get_whole_file_path(folder: &str, file_name: &str, extension: &str) -> PathBuf {
        let mut input_file_path = PathBuf::from(folder);
        input_file_path.push(file_name);
        input_file_path.set_extension(extension);
        input_file_path
    }
}

pub mod input_conversion {
    use std::str;

    pub fn input_to_lines<T: str::FromStr>(input: &str) -> Result<Vec<T>, <T as str::FromStr>::Err> {
        let mut lines = Vec::new();
        for line in input.lines() {
            let parsed = str::parse::<T>(line)?;
            lines.push(parsed)
        }
        Ok(lines)
    }
}

pub mod performance {
    use std::collections::HashMap;
    pub struct Cached<T, U, V>
    where T: Fn(U) -> V,
    {
        calculation: T,
        cache: HashMap<U, V>,
    }

    impl<T, U, V> Cached<T, U, V>
    where T: Fn(U) -> V,
          U: std::cmp::Eq, U: std::hash::Hash, U: std::clone::Clone,
    {
        pub fn new(calculation: T) -> Cached<T, U, V> {
            Cached { calculation, cache: HashMap::new() }
        }

        pub fn calculate(&mut self, argument: U) -> &V {
            if !self.cache.contains_key(&argument) {
                self.cache.insert(argument.clone(), (self.calculation)(argument.clone()));
            }
            self.cache.get(&argument).unwrap()
        }
    }

}