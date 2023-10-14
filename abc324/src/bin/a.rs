fn main() {
    let n_u32: u32 = input_u32();
    let a_vec_u32: Vec<u32> = input_vector_u32();

    for a in a_vec_u32.iter() {
        if a_vec_u32[0] != *a {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

////////////////////////////////////////////////////////////////////////////////

fn input_vector_u32() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

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
