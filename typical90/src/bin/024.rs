fn main() {
    let n_k_tuple: (u32, u32) = input_tuple();
    let a_vec_i32: Vec<i32> = input_vector_i32();
    let b_vec_i32: Vec<i32> = input_vector_i32();

    let counts: u32 = get_calculate_counts(n_k_tuple.0, &a_vec_i32, &b_vec_i32);
    let output_bool: &str = check_counts(n_k_tuple.1, counts);
    println!("{}", output_bool);
}

////////////////////////////////////////////////////////////////////////////////

fn check_counts(k: u32, counts: u32) -> &'static str {
    let mut return_str: &str = "Yes";
    if (k - counts) % 2 == 1 {
        return_str = "No";
    }
    if k < counts {
        return_str = "No";
    }
    return return_str;
}

fn get_calculate_counts(n: u32, a: &Vec<i32>, b: &Vec<i32>) -> u32 {
    let mut return_counts: u32 = 0;
    for i in 0..n as usize {
        return_counts = return_counts + (a[i] - b[i]).abs() as u32;
    }

    return return_counts;
}

fn input_tuple() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_vector_i32() -> Vec<i32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<i32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
