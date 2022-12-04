use std::fs;

fn main() {
    let file_path = "data.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let data: Vec<&str> = contents.split('\n').collect();

    // Init a counter at 0 to represent the current elf
    let mut counter = 0;

    // Iterate through the data Vector
    // Fold the data Vector into a result Vector of discreet totals, with the total resetting to 0 if "" is encountered
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    let res_arr: Vec<u32> = data.iter().fold(vec![0], |mut acc, curr| {
        // Check if the value is an empty string
        match curr {
            // If empty string, push a new value: u32 of 0 into the Vector
            &"" => {
                acc.push(0);
                // Increment the counter
                counter += 1;
                acc
            }
            // Else add the current value to the value at the index represented by the counter in the result Vector
            _ => {
                let num: u32 = curr.parse().unwrap();
                acc[counter] += num;
                acc
            }
        }
    });

    // Init a mutable index variable to 0
    let mut index: usize = 0;
    // Init a mutable largest variable to 0
    let mut largest: u32 = 0;
    // Iterate through the result vector
    for (i, num) in res_arr.iter().enumerate() {
        // If the value is larger than the largest variable, assign the value to the largest variable
        if num > &largest {
            largest = *num;
            index = i;
        }
        // Else continue
    }
    // print index + 1
    index += 1;
    println!("index: {index}");
    println!("largest: {largest}");
}
