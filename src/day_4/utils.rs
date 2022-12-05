use std::fs;

fn read_data(path: &str) -> String {
    let result = fs::read_to_string(path).unwrap();
    result
}

fn convert_to_nums(entry: Vec<&str>) -> Vec<Vec<usize>> {
    let first_range = entry[0].split("-").collect::<Vec<&str>>();
    let first_num_range = first_range.iter().map(|string|string.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let second_range = entry[1].split("-").collect::<Vec<&str>>();
    let second_num_range = second_range.iter().map(|string|string.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    vec!(first_num_range, second_num_range)


}

fn check_range(entry: Vec<&str>) -> usize {
    let mut numeric_entry = convert_to_nums(entry);
    numeric_entry.sort();

    let first_range = &numeric_entry[0];
    let second_range = &numeric_entry[1];

    // First range starts before second range
    // If second range ends before first range, second range is within first range.
    // If both ranges start at the same number, one is always within the other
    // If both ranges end at the same number, one is always within the other
    let response = (
        second_range[1] <= first_range[1]
            || second_range[0] == first_range[0]
            || second_range[1] == first_range[1]
    ) as usize;
    response
}

fn overlap_at_all(entry: Vec<&str>) -> usize {
    let mut numeric_entry = convert_to_nums(entry);
    numeric_entry.sort();

    let first_range = &numeric_entry[0];
    let second_range = &numeric_entry[1];

    // First range starts before second range
    // If second range ends before first range, second range is within first range.
    // If both ranges start at the same number, one is always within the other
    // If both ranges end at the same number, one is always within the other
    // If second starts before first ends, they overlap

    // Put more shortly, if second range starts before first range ends, there is overlap
    let response = (
        // second_range[1] <= first_range[1]
        //     || second_range[0] == first_range[0]
        //     || second_range[1] == first_range[1]
        //     || second_range[0] < first_range[1]
        second_range[0] <= first_range[1]
    ) as usize;
    response
}

pub fn part_1() {
    let data = read_data("src/day_4/my_data.txt");
    let data_pairs = data.lines().map(
        |line|line.split(",").collect::<Vec<&str>>()
    );
    let overlaps = data_pairs.map(|entry|check_range(entry)).collect::<Vec<usize>>();
    let overlap_count: usize = overlaps.iter().sum();
    print!("Part 1 solution: {} pairs overlap\n", overlap_count.to_string());
}

pub fn part_2() {
    let data = read_data("src/day_4/my_data.txt");
    let data_pairs = data.lines().map(
        |line|line.split(",").collect::<Vec<&str>>()
    );
    let overlaps = data_pairs.map(|entry|overlap_at_all(entry)).collect::<Vec<usize>>();
    let overlap_count: usize = overlaps.iter().sum();
    print!("Part 2 solution: {} pairs overlap\n", overlap_count.to_string());
}