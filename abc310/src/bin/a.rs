fn main() {
    let u32_tuple_n_p_q = input_u32_tuple();
    let u32_vec_d = input_u32_vector();
    let mut min_price = u32_tuple_n_p_q.1;

    for i in 0..u32_tuple_n_p_q.0 {
        if min_price > u32_vec_d[i as usize] + u32_tuple_n_p_q.2 {
            min_price = u32_vec_d[i as usize] + u32_tuple_n_p_q.2;
        }
    }
    println!("{}", min_price);
}

////////////////////////////////////////////////////////////////////////////////
fn input_u32_tuple() -> (u32, u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1], v[2]);
}

fn input_u32_vector() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
