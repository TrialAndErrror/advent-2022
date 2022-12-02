use std::fs;

pub struct ElfCalories {
    pub input: Vec<u64>,
    pub total_calories: u64,
}

fn make_num_vector_from_array(input: String) -> Vec<u64> {
    input.split(
        "\n"
    ).map(
        |num| {
            let new_string = String::from(num);
            let result = new_string.parse::<u64>().unwrap();
            result
        }
    ).collect::<Vec<u64>>()
}

impl ElfCalories {
    pub fn new(input: &str) -> ElfCalories {
        let input_string = String::from(input);
        let entry_vector = make_num_vector_from_array(input_string);
        let total_calories = entry_vector.iter().sum();

        let obj = ElfCalories {
            input: entry_vector,
            total_calories,
        };
        obj
    }
}

fn read_data() -> String {
    let result = fs::read_to_string("src/day_1/my_data.txt").unwrap();
    result
}

pub fn parse_data() -> Vec<ElfCalories> {
    let data = read_data();
    let string_vector = data.split("\n\n").collect::<Vec<&str>>();
    let elf_array = string_vector.iter().map(|entry| ElfCalories::new(entry)).collect::<Vec<ElfCalories>>();
    elf_array
}