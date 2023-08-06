fn main() {
    let h_w_tuple: (u32, u32) = input_tuple();
    if h_w_tuple.0 != 1 && h_w_tuple.1 != 1 {
        println!("{}", ((h_w_tuple.0 + 1) / 2) * ((h_w_tuple.1 + 1) / 2));
    } else {
        println!("{}", h_w_tuple.0 * h_w_tuple.1);
    }
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
