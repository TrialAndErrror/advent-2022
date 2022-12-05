use std::fs;

const ALL_LETTERS: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn read_data() -> String {
    let result = fs::read_to_string("src/day_3/my_data.txt").unwrap();
    result
}

pub fn split_into_pockets(entry: &str) -> (&str, &str) {
    let length = entry.len();
    let midpoint = length / 2;
    entry.split_at(midpoint)
}

pub fn find_common_character((pack_1, pack_2): (&str, &str)) -> char {
    for character in pack_1.chars() {
        if pack_2.contains(character) {
            return character
        }
    };
    panic!("No common character found!");
}

pub fn get_value(check_char: char) -> usize {
    ALL_LETTERS.chars().position(|x| x == check_char).unwrap()
}

pub fn find_group_identifier(&(pack_1, pack_2, pack_3): &(&str, &str, &str)) -> char {
    for character in pack_1.chars() {
        if pack_2.contains(character) && pack_3.contains(character) {
            return character
        }
    };
    panic!("No common character found!");
}