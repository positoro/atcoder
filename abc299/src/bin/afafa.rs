fn main() {
    todo!();
}

////////////////////////////////////////////////////////////////////////////////

fn input_b_c() -> (u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    return input_strings;
}

fn input_a() -> u16 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
