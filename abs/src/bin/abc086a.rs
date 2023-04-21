fn main() {
    let (input_a, input_b) = input_a_b();
    if (input_a * input_b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_a_b() -> (u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
