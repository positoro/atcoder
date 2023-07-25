fn main() {
    let n_u32: u32 = input_u32();
    let a_vec_u32: Vec<u32> = input_vector_u32();
    let q_u32: u32 = input_u32();
    let b_vec_u32: Vec<u32> = input_vector_u32_low(&q_u32);

    println!("{:?}", n_u32);
    println!("{:?}", a_vec_u32);
    println!("{:?}", q_u32);
    println!("{:?}", b_vec_u32);
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

fn input_vector_u32_low(low: &u32) -> Vec<u32> {
    let mut return_vector: Vec<u32> = Vec::new();
    for _ in 0..*low {
        return_vector.push(input_u32());
    }

    return return_vector;
}
