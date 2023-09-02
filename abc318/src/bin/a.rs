fn main() {
    let n_m_p_tuple_u32: (u32, u32, u32) = input_tuple_u32();
    let mut counter: u32 = 0;
    while n_m_p_tuple_u32.0 >= n_m_p_tuple_u32.1 + counter * n_m_p_tuple_u32.2 {
        counter = counter + 1;
    }
    println!("{}", counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_tuple_u32() -> (u32, u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1], v[2]);
}
