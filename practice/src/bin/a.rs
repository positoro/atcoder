fn main() {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let a: u16 = input_strings.trim().parse().ok().unwrap();

    input_strings = "".to_string();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    let b: u16 = v[0];
    let c: u16 = v[1];

    input_strings = "".to_string();
    std::io::stdin().read_line(&mut input_strings).ok();
    let s: &str = input_strings.trim();

    println!("{} {}", a + b + c, s);
}
