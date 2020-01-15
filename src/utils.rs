pub mod file_handling {
    use std::{io, env, fs, path::PathBuf, ffi::OsStr};

    pub fn get_input_filename(day_number: u32) -> String {
        format!("day_{:0>2}", day_number)
    }

    pub fn get_input_extension() -> String {
        String::from("input")
    }

    pub fn get_test_result_suffix() -> String {
        String::from("result")
    }

    pub struct InputWithResult {
        pub input: String,
        pub first_star_solution: Option<String>,
        pub second_star_solution: Option<String>,
    }

    pub fn get_input_for_day(day_number: u32, folder: &str) -> Result<String, io::Error> {
        let path = get_whole_file_path(folder, &get_input_filename(day_number), &get_input_extension());
        fs::read_to_string(path)
    }

    pub fn get_test_inputs_with_results_for_day(day_number: u32, folder: &str) -> Result<Vec<InputWithResult>, io::Error> {
        let mut inputs = Vec::new();
        for mut file_path in list_test_input_file_names(day_number, folder)? {
            file_path.set_extension(get_input_extension());
            let input = fs::read_to_string(&file_path)?;

            file_path.set_extension(get_test_result_suffix());
            let results = fs::read_to_string(&file_path)?;

            let mut results = results.lines().map(str::to_string);
            let first_star_solution = results.next();
            let second_star_solution = results.next();

            inputs.push(InputWithResult { input, first_star_solution, second_star_solution });
        }
        Ok(inputs)
    }

    fn list_test_input_file_names(day_number: u32, folder: &str) -> Result<Vec<PathBuf>, io::Error> {
        let test_input_filename = format!("{}", get_input_filename(day_number));
        let mut filenames = Vec::new();

        for item in fs::read_dir(folder)? {
            let path = item?.path();

            // Check that the current item is a file with the proper name and suffix (expected result file).
            let is_correct_day = path.file_name().and_then(OsStr::to_str).map(|x| {x.contains(&get_input_filename(day_number))}).unwrap_or(false);
            let has_correct_extension = path.extension().and_then(OsStr::to_str).map(|x| { x == get_test_result_suffix() }).unwrap_or(false);
            if !is_correct_day || !has_correct_extension {
                continue;
            }

            let mut input_path = path.clone();
            input_path.set_extension("");
            filenames.push(input_path);
        }
        Ok(filenames)
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
