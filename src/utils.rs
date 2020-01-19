pub mod file_handling {
    use std::{io, fs, path::PathBuf};

    pub fn get_input_filename(day_number: u32) -> String {
        format!("day_{:0>2}", day_number)
    }

    pub fn get_input_extension() -> String {
        String::from("input")
    }

    pub fn get_input_for_day(day_number: u32, folder: &str) -> Result<String, io::Error> {
        let path = get_whole_file_path(folder, &get_input_filename(day_number), &get_input_extension());
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
