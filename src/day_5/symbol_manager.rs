use crate::day_5::utils;
use crate::day_5::utils::split_by_four;

#[derive(Debug)]
pub struct SymbolManager {
    pub data: Vec<Vec<String>>,
}

impl SymbolManager {
    pub fn from_data(data: Vec<&str>) -> SymbolManager {
        println!("Making Symbol Manager...");

        let data_lists = data.iter().map(|line| split_by_four(*line)).collect::<Vec<Vec<String>>>();
        let number_of_stacks = data_lists.last().unwrap().len();

        SymbolManager {
            data: utils::make_stacks_from_data(data_lists, number_of_stacks)
        }
    }

    fn process_instruction(&mut self, instruction: String) {
        let info = instruction.split(" ").collect::<Vec<&str>>();

        let source_row = info[3].parse::<usize>().unwrap() - 1;
        let count_to_move = info[1].parse::<usize>().unwrap();
        let dest_row = info[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count_to_move {
            let value_option = self.data[source_row].pop();
            match value_option {
                Some(value) => {
                    if value.len() > 0 {
                        self.data[dest_row].push(value);
                    } else {
                        break
                    }
                },
                None => {
                    break
                }
            };
        }
    }

    pub fn process_all_instructions(&mut self, instructions: Vec<String>) {
        println!("Processing instructions...");
        for instruction in instructions {
            println!("Processing {}", instruction);
            self.process_instruction(instruction);
        }
    }
    pub fn print_tops(&self) {
        let all_tops = &self.data.iter().map(
            |entry| utils::get_number_from_symbol(entry.last().unwrap())
        ).collect::<Vec<String>>();
        println!("{}", all_tops.join(""))
    }
}


#[derive(Debug)]
pub struct SymbolManager9001 {
    pub data: Vec<Vec<String>>,
}

impl SymbolManager9001 {
    pub fn from_data(data: Vec<&str>) -> SymbolManager {
        println!("Making Symbol Manager...");

        let data_lists = data.iter().map(|line| split_by_four(*line)).collect::<Vec<Vec<String>>>();
        let number_of_stacks = data_lists.last().unwrap().len();

        SymbolManager {
            data: utils::make_stacks_from_data(data_lists, number_of_stacks)
        }
    }

    fn process_instruction(&mut self, instruction: String) {
        let info = instruction.split(" ").collect::<Vec<&str>>();

        let source_row = info[3].parse::<usize>().unwrap() - 1;
        let count_to_move = info[1].parse::<usize>().unwrap();
        let dest_row = info[5].parse::<usize>().unwrap() - 1;

        let source_copy = self.data[source_row].clone()
        let v = source_copy.as_slice();
        let split_index = v.len() - count_to_move;

        self.data[source_row] = v[..split_index].to_vec();
        self.data[dest_row].clone().extend(v[split_index..].to_vec());
    }

    pub fn process_all_instructions(&mut self, instructions: Vec<String>) {
        println!("Processing instructions...");
        for instruction in instructions {
            println!("Processing {}", instruction);
            self.process_instruction(instruction);
        }
    }
    pub fn print_tops(&self) {
        let all_tops = &self.data.iter().map(
            |entry| utils::get_number_from_symbol(entry.last().unwrap())
        ).collect::<Vec<String>>();
        println!("{}", all_tops.join(""))
    }
}