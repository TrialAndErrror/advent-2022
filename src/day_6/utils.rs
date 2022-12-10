use std::fs;
use std::collections::HashSet;

pub fn read_data(path: &str) -> String {
    let result = fs::read_to_string(path).unwrap();
    result
}

pub fn is_start_of_packet(characters: &str) -> bool {
    let result = HashSet::<char>::from_iter(characters.chars()).len() == 4;
    result
}

pub fn is_start_of_message(characters: &str, message_length: usize) -> bool {
    let result = HashSet::<char>::from_iter(characters.chars()).len() == message_length;
    result
}