fn main() {
    let (input_n, input_a, input_b) = input_n_a_b();
    let input_c_u16_vector = input_u16_vector();
    let a_plus_b = input_a + input_b;

    for i in 0..input_n {
        if input_c_u16_vector[i as usize] == a_plus_b {
            println!("{}", i + 1);
            break;
        } else {
            continue;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_u16_vector() -> Vec<u16> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

fn input_n_a_b() -> (u16, u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1], v[2]);
}
