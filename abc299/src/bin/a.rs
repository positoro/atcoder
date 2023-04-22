fn main() {
    let input_n = input_n();
    let input_s = input_s();

    let star_index = input_s.find('*').unwrap();
    let first_gate_index = input_s.find('|').unwrap();
    let last_gate_index = input_s.rfind('|').unwrap();

    if first_gate_index < star_index && star_index < last_gate_index {
        println!("in");
    } else {
        println!("out");
    }
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

fn input_n() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
