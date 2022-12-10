use std::fs;

pub fn read_data(path: &str) -> String {
    let result = fs::read_to_string(path).unwrap();
    result
}

pub fn clean_string_from(entry: String) -> String {
    String::from(entry.trim())
}

pub fn split_by_four(line: &str) -> Vec<String> {
    println!("Splitting line: {}", line);
    let mut data_vec: Vec<String> = vec![];
    let length = line.len();
    let num_iters = line.len() as usize / 4;
    if num_iters > 0 {
        for num in 0..num_iters + 1 {
            let start = num * 4;
            let end = {
                let calc = (num + 1) * 4;
                if calc > length {
                    length
                } else {
                    calc
                }
            };
            let current_segment = String::from(&line[start..end]);
            println!("{}", current_segment);
            data_vec.push(clean_string_from(current_segment));
        }
    }

    data_vec
}

pub fn make_stacks_from_data(data_lists: Vec<Vec<String>>, number_of_stacks: usize) -> Vec<Vec<String>> {
    let mut all_data: Vec<Vec<String>> = vec![vec![]];
    for num in 0..number_of_stacks {
        let stack_data = data_lists.iter().map(|line|line[num].clone()).collect::<Vec<String>>();
        if stack_data.len() > 0 {
            let mut stack_data = stack_data.into_iter().filter(|entry|entry != &String::from("")).collect::<Vec<String>>();
            stack_data.reverse();
            all_data.push(stack_data)
        }
    };
    all_data.remove(0);
    all_data
}

pub fn get_number_from_symbol(symbol: &str) -> String {
    String::from(symbol.chars().collect::<Vec<char>>()[1])
}
