use std::fs;

fn get_move_index(move_selections: Vec<&str>, text: &str) -> usize {
    move_selections.iter().position(|&x| x == text).unwrap()
}

pub fn get_points(line: &str) -> i16 {
    let mut moves = line.split(" ");

    let opp_moves = Vec::from(["A", "B", "C"]);
    let my_moves = Vec::from(["X", "Y", "Z"]);

    let opp_move = get_move_index(opp_moves, moves.next().unwrap());
    let my_move = get_move_index(my_moves, moves.next().unwrap());

    let game_outcome = my_move as i16 - opp_move as i16;

    let my_game_points: i16 = match game_outcome {
        0 => 3,
        1 => 6,
        2 => 0,
        -1 => 0,
        -2 => 6,
        _ => panic!("Invalid game outcome")
    };
    let total_points = my_game_points as i16 + my_move as i16 + 1;
    total_points
}

pub fn get_my_move(line: &str) -> i16 {
    let mut moves = line.split(" ");

    let opp_moves = Vec::from(["A", "B", "C"]);
    let my_moves = Vec::from(["X", "Y", "Z"]);

    let opp_text = moves.next().unwrap();
    let opp_move = get_move_index(opp_moves, opp_text);

    let my_goal = moves.next().unwrap();
    let my_move = match my_goal {
        "X" => (opp_move + 2) % 3,
        "Y" => opp_move,
        "Z" => (opp_move + 1) % 3,
        _ => panic!("Invalid Goal!")
    };
    let my_text = my_moves[my_move];

    let overall_move = String::from(opp_text) + " " + my_text;
    get_points(&overall_move)
}


pub fn read_data() -> String {
    let result = fs::read_to_string("src/day_2/my_data.txt").unwrap();
    result
}