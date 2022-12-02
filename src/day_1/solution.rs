use super::my_data::{parse_data};

pub fn part_1() {
    println!("Part 1: \n");
    let data = parse_data();
    let max = data.iter().max_by_key(|elf| elf.total_calories);
    match max {
        None => panic!("Max value not found"),
        Some(most_prepared_elf) => {
            println!("Max Value Found!");
            println!("Total Calories: {}", most_prepared_elf.total_calories);
            println!("List of Entries: {:?}", most_prepared_elf.input);
        }
    }
}


pub fn part_2() {
    println!("\nPart 2: \n");
    let data = parse_data();
    let mut elf_calories_vec = data.iter().map(|elf|elf.total_calories).collect::<Vec<u64>>();
    elf_calories_vec.sort();
    elf_calories_vec.reverse();
    let top_3 = &elf_calories_vec[..3];
    let total_from_top_3: u64 = top_3.iter().sum();
    println!("Total Calories from Top 3 Elves: {}", total_from_top_3)
}