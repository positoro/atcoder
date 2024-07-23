fn main() {
    let k_u32: u32 = input_u32();
    let answer: u32;

    if k_u32 <= 99 {
        answer = 100 - k_u32;
    } else if k_u32 <= 199 {
        answer = 200 - k_u32;
    } else {
        answer = 300 - k_u32;
    }

    println!("{}", answer);
}

///////////////////////////////////////////////////////////////////////////////

fn input_u32() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
