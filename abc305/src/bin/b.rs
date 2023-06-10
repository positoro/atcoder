fn main() {
    let input_char_p_q = input_char_tuple();
    let p_distance: i32 = get_distance(input_char_p_q.0);
    let q_distance: i32 = get_distance(input_char_p_q.1);

    let distance = (p_distance - q_distance).abs();
    println!("{}", distance);
}

////////////////////////////////////////////////////////////////////////////////
fn get_distance(input_char: char) -> i32 {
    let mut return_distance = 0;
    if input_char == 'A' {
        return_distance = 0;
    } else if input_char == 'B' {
        return_distance = 3;
    } else if input_char == 'C' {
        return_distance = 4;
    } else if input_char == 'D' {
        return_distance = 8;
    } else if input_char == 'E' {
        return_distance = 9;
    } else if input_char == 'F' {
        return_distance = 14;
    } else if input_char == 'G' {
        return_distance = 23;
    }

    return return_distance;
}

fn input_char_tuple() -> (char, char) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<char> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
