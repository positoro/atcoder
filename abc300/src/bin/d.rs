fn main() {
    let input_n = input_n();

    println!("{}", input_n);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n() -> u64 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u64> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
