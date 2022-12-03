use super::utils::{read_data, get_points, get_my_move};

pub fn part_1() -> i16 {
    let all_input: String = read_data();
    let round_points = all_input.lines().map(|entry| get_points(entry)).collect::<Vec<i16>>();
    let total_points: i16 = round_points.iter().sum();
    println!("Part 1 Total Points: {}", total_points);  // 11767
    total_points
}

pub fn part_2() -> i16 {
    let all_input: String = read_data();
    let round_points = all_input.lines().map(|entry| get_my_move(entry)).collect::<Vec<i16>>();
    let total_points: i16 = round_points.iter().sum();
    println!("Part 2 Total Points: {}", total_points); // 13886
    total_points
}