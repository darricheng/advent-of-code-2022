use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Split string into a vec of pairs
    let data: Vec<&str> = contents.split('\n').collect();

    // Split each str into a vec of 2 letters
    let fmt_data: Vec<Vec<&str>> = data.iter().map(|str| str.split(' ').collect()).collect();

    // Fold (reduce) vec of vec into a single var with the final score, init accumulator as 0
    let score = fmt_data.iter().fold(0, |acc, curr| {
        // Rock: 0, Paper: 1, Scissors: 2
        // Match the opponent's value
        let opp = match curr[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!(),
        };
        // Match the player's value
        let player = match curr[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!(),
        };
        // Formula:
        // * If (player - opponent === 0) DRAW
        if player - opp == 0 {
            return acc + player + 3;
        }
        // * (Player - Opponent (+3 if negative)) % 2 === 0 ? Opponent wins : Player wins
        let test_val = if player - opp < 0 {
            player - opp + 3
        } else {
            player - opp
        };
        // Calculate the outcome
        if test_val % 2 == 0 {
            acc + player
        }
        // Return accumulator + outcome
        else {
            acc + player + 6
        }
    });

    // Print the final score
    println!("{score}")
}
