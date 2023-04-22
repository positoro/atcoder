fn main() {
    let input_n = input_n();
    let input_s = input_s();
    let splitted_s_vec: Vec<&str> = input_s.trim().split('-').collect();

    let max = splitted_s_vec.iter().max().unwrap().len();
    if max == 0 || input_s.trim().find('-').is_some() == false {
        println!("-1");
    } else {
        println!("{}", max);
    }
}

////////////////////////////////////////////////////////////////////////////////

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
