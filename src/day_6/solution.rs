use super::utils::{read_data, is_start_of_packet, is_start_of_message};

pub fn part_1() {
    let data = read_data("src/day_6/my_data.txt");
    let num_options = data.len();
    let position: Option<usize> = {
        let mut result = None;
        for current_num in 0..num_options - 3 {
            let check = is_start_of_packet(&data[current_num..current_num + 4]);
            if check {
                result = Some(current_num + 4);
                break
            };
        };
        result
    };

    match position {
        Some(value) => println!("Start sequence found at {}: {}", value.to_string(), &data[value..value + 4]),
        None => panic!("Start sequence not found")
    }
}

pub fn part_2() {
    let data = read_data("src/day_6/my_data.txt");
    let num_options = data.len();
    let message_length: usize = 14;

    let position: Option<usize> = {
        let mut result = None;
        for current_num in 0..num_options - 3 {
            let check = is_start_of_message(&data[current_num..current_num + message_length], message_length);
            if check {
                result = Some(current_num + message_length);
                break
            };
        };
        result
    };

    match position {
        Some(value) => println!("Message sequence found at {}: {}", value.to_string(), &data[value..value + message_length]),
        None => panic!("Message sequence not found")
    }
}