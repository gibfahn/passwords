pub fn main() {
    let results = test_all(4);
    println!("\nResults: {:?}", results );
}

const CH: [char; 40] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '.', ',', '-', '!']; // Array of allowed chars


pub fn test_all(length: usize) -> Vec<String> {
    let mut output = Vec::new(); // Vec of okay passwords (Strings)
    let mut v = Vec::new(); // Vector of passwords as numbers (corresponding to chars in CH)
    let maxval = CH.len() - 1; // Maximum index in CH
    for _ in 0..length {
        v.push(0); // create vector of lowest values
    }
    println!("\n Checking length {}, eg: {:?}", length, v); // Show Vector length (debug)
    'outer: loop {
        if v[0] != maxval {
            v[0] += 1; // Increment the first number
        } else {
            let mut i = 0;
            while v[i] == maxval {
                if i == length - 1 { break 'outer; } // Checked all permutations so stop looping
                v[i] = 0; // Number resets, maxval -> zero
                i += 1;   // Carry the 1
            }
            v[i] += 1; // Increment first value that isn't maxval
        }
        //print!("{:?} ", v); // Show each tried combination (debugging only)
        if let Some(s) = test(&v) { output.push(s); }
    }
    if length > 1 {
        for i in test_all(length-1){ output.push(i); } // If password matches add to output
    }
    return output; // Return all matching strings
}

pub fn test(v: &Vec<usize>) -> Option<String> {
    let mut output = String::new();
    for &i in v { output.push(CH[i]); } // Convert vector of indexes to string of chars (CH=hashmap)
    let good_strings = vec!["as","p0","4"]; // Would be a hash result
    let valid: bool;
    {
        let output_ref: &str = &output; // Need to finish this borrow before we return output
        valid = good_strings.contains(&output_ref); // You'd check the hash here
    }
    if valid { Some(output) } else { None } // If the string is good, return it
}
