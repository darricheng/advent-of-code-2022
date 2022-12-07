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
        // Rock: 1, Paper: 2, Scissors: 3
        // Match the opponent's value
        let opp = match curr[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!(),
        };
        // Match the outcome
        let points = match curr[1] {
            "X" => {
                // Minus 1 from the opp's value
                // If player value is < 1, add 3 to it
                // Return the value
                if opp - 1 < 1 {
                    opp + 2
                } else {
                    opp - 1
                }
            }
            "Y" => opp + 3,
            "Z" => {
                // Add 1 to the opp's value
                // If player value is > 3, subtract 3 from it
                // Return the value + 6
                6 + if opp + 1 > 3 { opp - 2 } else { opp + 1 }
            }
            _ => panic!(),
        };

        acc + points
    });

    // Print the final score
    println!("{score}")
}
