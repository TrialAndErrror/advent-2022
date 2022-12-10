use super::symbol_manager::{SymbolManager, SymbolManager9001};
use super::utils::read_data;

pub fn part_1() {
    let data = read_data("src/day_5/stack.txt");
    let lines = data.lines().collect::<Vec<&str>>();
    let instructions = read_data("src/day_5/my_data.txt").lines().map(|line|String::from(line)).collect::<Vec<String>>();
    let mut manager = SymbolManager::from_data(lines);
    manager.process_all_instructions(instructions);
    manager.print_tops()
}

pub fn part_2() {
    let data = read_data("src/day_5/stack.txt");
    let lines = data.lines().collect::<Vec<&str>>();
    let instructions = read_data("src/day_5/my_data.txt").lines().map(|line|String::from(line)).collect::<Vec<String>>();
    let mut manager = SymbolManager9001::from_data(lines);
    manager.process_all_instructions(instructions);
    manager.print_tops()
}