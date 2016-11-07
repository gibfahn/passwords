#![feature(inclusive_range_syntax)]
use std::ops::Deref;
pub fn main() {
    let results = test_all(4);
    println!("\nResults: {:?}", results );
}

const CH: [char; 40] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '.', ',', '-', '!']; // Array of allowed chars

pub fn test_all(length: u32) -> Vec<String> {
    (0...(CH.len().pow(length))).filter(test).map(convert).collect::<Vec<String>>()
}

pub fn test(v: &usize) -> bool {
    let output = convert(*v);
    let good_strings = vec!["as","p0","4"]; // Would be a hash result
    good_strings.contains(&output.deref()) // You'd check the hash here
}

pub fn convert(mut v: usize) -> String {
    let mut output = String::new();
    let length = CH.len();
    while v > 0 {
        output.push(CH[v%length]);
        v /= length;
    }
    output
}
