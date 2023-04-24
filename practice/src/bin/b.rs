fn main() {
    let (input_n, input_q) = input_usizen_u16q();
    let label: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let string_for_sort: String = label[0..input_n].to_string();
    let answer = quick_sort(string_for_sort);
    println!("! {}", answer);
}

////////////////////////////////////////////////////////////////////////////////
fn quick_sort(string_for_sort: String) -> String {
    let pivot_index = 0;
    let mut left: String = String::new();
    let mut right: String = String::new();
    if string_for_sort.len() <= 1 {
        return string_for_sort;
    }

    let char_of_pivot_index = string_for_sort.chars().nth(pivot_index).unwrap();

    for i in 1..string_for_sort.len() {
        let char_of_index = string_for_sort.chars().nth(i).unwrap();
        if interactive_query(char_of_pivot_index, char_of_index) == '<' {
            right.push(char_of_index);
        } else {
            left.push(char_of_index);
        }
    }
    left = quick_sort(left);
    right = quick_sort(right);
    let return_string: String = left + &char_of_pivot_index.to_string() + &right;
    return return_string;
}
////////////////////////////////////////////////////////////////////////////////
fn interactive_query(c1: char, c2: char) -> char {
    let mut input_strings = String::new();
    println!("? {} {}", c1, c2);
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<char> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}

fn input_usizen_u16q() -> (usize, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0] as usize, v[1] as u16);
}
