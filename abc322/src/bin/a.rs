fn main() {
    let s_u32: u32 = input_u32();
    let s_string: String = input_s();
    let index = s_string.find("ABC");
    let mut output: i32 = -1;

    if index.is_none() == false {
        output = index.unwrap() as i32 + 1;
    }

    println!("{:?}", output);
}

////////////////////////////////////////////////////////////////////////////////

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}

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
