fn main() {
    let a_b_tuple_u32: (u32, u32) = input_tuple();

    println!(
        "{:?}",
        a_b_tuple_u32.0.pow(a_b_tuple_u32.1) + a_b_tuple_u32.1.pow(a_b_tuple_u32.0)
    );
}

////////////////////////////////////////////////////////////////////////////////

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
