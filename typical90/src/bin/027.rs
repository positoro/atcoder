use std::collections::HashMap;

fn main() {
    let n_u32: u32 = input_u32();
    let s_vec_string: Vec<String> = input_string_vector(n_u32 as usize);
    let mut accounts_hash: HashMap<String, bool> = HashMap::new();
    let mut output_vec_u32: Vec<u32> = Vec::new();
    for i in 0..n_u32 {
        if accounts_hash.contains_key(&s_vec_string[i as usize]) == false {
            accounts_hash.insert(s_vec_string[i as usize].clone(), true);
            output_vec_u32.push(i + 1);
        }
    }
    output_vec_u32.sort();
    for o in output_vec_u32.iter() {
        println!("{}", o);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_u32() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}

fn input_string_vector(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}
