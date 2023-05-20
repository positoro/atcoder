fn main() {
    let (input_h, input_w) = input_h_w();
    let mut input_string_vector = input_string_vector(input_h);
    let c: char = get_char_from_string_vector(0, 0, input_string_vector);

    println!("{}", c);
}

////////////////////////////////////////////////////////////////////////////////

fn get_char_from_string_vector(h: usize, w: usize, string_vector: Vec<String>) -> char {
    let return_char: char = string_vector[h].chars().nth(w).unwrap();
    return return_char;
}

fn input_string_vector(low: u16) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for l in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}

fn input_h_w() -> (u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
