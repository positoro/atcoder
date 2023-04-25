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

pub fn for_test_bench_a() {
    let mut counter = 0;
    for i in 0..10 {
        counter += 1;
    }
}

pub fn for_test_bench_b() {
    let mut counter = 0;
    for i in 0..10 {
        counter *= 2;
    }
}
