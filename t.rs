#![feature(inclusive_range_syntax)]
pub fn main() {
    let results = test_all(3);
    println!("\nResults: {:?}", results );
}

pub fn test_all(maxlen: u32) -> Vec<Vec<u32>> {
    // let chars = "abcdefghijklmnopqrstuvwxyz0123456789.,-!".chars();
    let mut output = Vec::new(); // Vec of okay outputs
    let mut v = Vec::new();
    for _ in 0..maxlen {
        v.push(0); // create vector of lowest values
    }
    println!("\n\n{:?}",v );
    'outer: loop {
        if v[0] != 9 {
            v[0] += 1;
        } else {
            let mut i = 0;
            while v[i] == 9 {
                if i == maxlen as usize - 1 { break 'outer; }
                v[i] = 0;
                i += 1;
            }
            v[i] += 1;
        }
        print!("{:?} ", v);
        if test(&v) { output.push(v.clone()) }
    }
    if maxlen > 1 {
        for i in test_all(maxlen-1){
            output.push(i);
        }
    }
    return output;
}

pub fn test(s: &Vec<u32>) -> bool {
    let good_strings = vec![vec![2],vec![3,4,1]];
    good_strings.contains(s)
}
