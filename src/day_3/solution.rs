use super::utils::{find_common_character, get_value, read_data, split_into_pockets, find_group_identifier};

pub fn part_1() -> () {
    let data = read_data();
    let pack_list = data.lines().map(|pack|split_into_pockets(pack)).collect::<Vec<(&str, &str)>>();
    let common_chars = pack_list.iter().map(|pack|find_common_character(*pack)).collect::<Vec<char>>();
    let values = common_chars.iter().map(|label_char|get_value(*label_char)).collect::<Vec<usize>>();
    let total: usize = values.iter().sum();
    println!("Part 1 Total Value: {}", total)
}


pub fn part_2() {
    let data = read_data();
    let mut total: usize = 0;
    {
        let mut lines = data.lines();
        let num_reps = lines.clone().count() / 3;

        for _ in 0..num_reps {
            let segment = (lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap());
            let group_badge = find_group_identifier(&segment);
            total = total + get_value(group_badge)
        }
    }

    println!("Part 2 Total Value: {}", total)

}