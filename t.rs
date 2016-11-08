use std::ops::Deref;
use rayon::par_iter::{IntoParallelIterator, ParallelIterator};
pub fn main() {
    let results = test_all(4);
    println!("\nResults: {:?}", results );
}

const CH_LEN: usize = 40;
const CH: [char; CH_LEN] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '.', ',', '-', '!']; // Array of allowed chars

pub fn test_all(length: u32) -> Vec<String> {
    (0..(CH.len().pow(length)+1)).into_par_iter().filter(test).map(convert).collect::<Vec<String>>()
}

pub fn test(num: &usize) -> bool {
    let output = convert(*num);
    let good_strings = vec!["as","p0","4"]; // Would be a hash result
    good_strings.contains(&output.deref()) // You'd check the hash here
}

pub fn convert(mut num: usize) -> String {
    let mut output = String::new();
    while num > 0 {
        output.push(CH[num % CH_LEN]);
        num /= CH_LEN;
    }
    output
}
