fn main() {
    let input_n: u8 = input_n();
    let mut input_u8vec_a: Vec<u8> = input_u8vec_a();
    let max: u8 = *input_u8vec_a.iter().max().unwrap();
    let index: usize = input_u8vec_a.iter().position(|&r| r == max).unwrap();

    println!("{}{:?} {} {}", input_n, input_u8vec_a, max, index);
}

////////////////////////////////////////////////////////////////////////////////

fn input_u8vec_a() -> Vec<u8> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let mut v: Vec<u8> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

fn input_n() -> u8 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let mut v: Vec<u8> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
