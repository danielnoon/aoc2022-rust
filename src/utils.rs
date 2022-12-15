use std::fs;

pub fn read_file(filename: &str) -> String {
    let data = fs::read_to_string(filename).expect("could not read file");
    return data.replace("\r", "");
}
