mod day_2;
mod day_1;
mod day_3;
mod day_4;
mod day_5;


fn main() {
    println!("Solutions for Day 1:");
    day_1::solution::part_1();
    day_1::solution::part_2();

    println!("\n\nSolutions for Day 2:");
    day_2::solution::part_1();
    day_2::solution::part_2();

    println!("\n\nSolutions for Day 3:");
    day_3::solution::part_1();
    day_3::solution::part_2();

    println!("\n\nSolutions for Day 4:");
    day_4::utils::part_1();
    day_4::utils::part_2();

    println!("\n\nSolutions for Day 5:");
    day_5::solution::part_1();
    day_5::solution::part_2();
}
