#![feature(inclusive_range_syntax)]
pub fn main() {
    let results = test_all(3);
    println!("\nResults: {:?}", results );
}

const CH: [char; 40] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '.', ',', '-', '!'];


pub fn test_all(maxlen: u32) -> Vec<String> {
    let maxval = CH.len() - 1;
    let mut output = Vec::new(); // Vec of okay outputs
    let mut v = Vec::new();
    for _ in 0..maxlen {
        v.push(0); // create vector of lowest values
    }
    println!("\n\n{:?}",v );
    'outer: loop {
        if v[0] != maxval {
            v[0] += 1;
        } else {
            let mut i = 0;
            while v[i] == maxval {
                if i == maxlen as usize - 1 { break 'outer; }
                v[i] = 0;
                i += 1;
            }
            v[i] += 1;
        }
        print!("{:?} ", v);
        if let Some(s) = test(&v) { output.push(s); }
    }
    if maxlen > 1 {
        for i in test_all(maxlen-1){
            output.push(i);
        }
    }
    return output;
}

pub fn test(v: &Vec<usize>) -> Option<String> {
    let mut s = String::new();
    for &i in v {
        s.push(CH[i]);
    }
    let valid: bool;
    let good_strings = vec!["as","p0","4"];
    {
        let s_ref: &str = &s;
        valid = good_strings.contains(&s_ref);
    }
    if valid { Some(s) } else { None }
}
