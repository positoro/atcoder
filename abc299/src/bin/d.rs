fn main() {
    let input_n = input_n();
    let mut left = 2;
    let mut right = input_n - 1;

    while left < right {
        let query_index = (left + right) / 2;
        let answer = input_interactive_si(query_index);
        if answer == 1 {
            right = query_index;
        } else {
            left = query_index + 1;
        }

        println!("{}", right - 1);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_interactive_si(i: u32) -> u32 {
    let mut input_strings = String::new();
    println!("? {}", i);
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
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
