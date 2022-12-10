use std::fs;

pub fn read_data(path: &str) -> String {
    let result = fs::read_to_string(path).unwrap();
    result
}

