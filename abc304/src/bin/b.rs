fn main() {
    let input_n = input_n();
    let mut print_n: u32 = input_n;

    if input_n <= 1000 - 1 {
        print_n = input_n;
    } else if input_n <= 10000 - 1 {
        print_n = (input_n / 10) * 10;
    } else if input_n <= 100000 - 1 {
        print_n = (input_n / 100) * 100;
    } else if input_n <= 1000000 - 1 {
        print_n = (input_n / 1000) * 1000;
    } else if input_n <= 10000000 - 1 {
        print_n = (input_n / 10000) * 10000;
    } else if input_n <= 100000000 - 1 {
        print_n = (input_n / 100000) * 100000;
    } else if input_n <= 1000000000 - 1 {
        print_n = (input_n / 1000000) * 1000000;
    }

    println!("{}", print_n);
}

////////////////////////////////////////////////////////////////////////////////
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
