use std::collections::HashMap;
#[allow(unused_variables)]
fn main() {
    let (input_n, input_k) = input_n_k();
    let input_a = input_a();

    let mut counter_hashmap: HashMap<u32, u32> = HashMap::new();
    let mut output_counter: u32 = 0;

    for num in input_a.iter() {
        let counter = counter_hashmap.entry(*num).or_insert(0);
        *counter += 1;
    }

    let mut counter_vec: Vec<(&u32, &u32)> = counter_hashmap.iter().collect();
    counter_vec.sort_by(|a, b| a.1.cmp(&b.1));
    if counter_vec.len() > input_k as usize {
        let counter_slice: &[(&u32, &u32)] = &counter_vec[0..counter_vec.len() - input_k as usize];
        for a in counter_slice.iter() {
            output_counter += a.1;
        }
    }
    println!("{}", output_counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_k() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_a() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
