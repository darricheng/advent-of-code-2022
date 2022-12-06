mod calc_alphas;
use crate::calc_alphas::calc_alphas;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "data.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Split string into a vec by rucksack
    let data: Vec<&str> = contents.split('\n').collect();

    // Assign a variable to the folded data vec
    let score = data.iter().fold(0, |acc, curr| {
        let first_half = curr[..(curr.len() / 2)].to_string();
        let second_half = curr[(curr.len() / 2)..].to_string();
        // For the first half of each rucksack, create a hashmap
        let first_half_arr: Vec<&str> = first_half.split("").collect();
        let obj =
            first_half_arr
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<&str, i32>, curr| {
                    // Assign each letter key a value of 1
                    acc.insert(curr, 1);
                    acc
                });

        // Iterate through the second half, store the repeated letter then break the loop
        let second_half_arr: Vec<&str> = second_half.split("").collect();
        let repeated_letter: &str = second_half_arr
            .iter()
            .find(|l| -> bool { obj.get(*l).unwrap_or(&-1) > &0 && !l.is_empty() })
            .unwrap();

        // use the calc_alphas fn to get the value of the letter
        // return the accumulator + letter value
        acc + calc_alphas(repeated_letter)
    });

    println!("{score}");
}
