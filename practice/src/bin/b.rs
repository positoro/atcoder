fn main() {
    let (input_n, input_q) = input_usizen_u16q();
    let label: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let string_for_sort: String = label[0..input_n].to_string();
    //    let answer = quick_sort(string_for_sort);
    let answer = merge_sort(string_for_sort);
    println!("! {}", answer);
}

/////////////////////////////////////////////////////////////////////////////////

fn merge_sort(string_for_sort: String) -> String {
    let pivot_index = string_for_sort.len() / 2;
    let mut left: String = string_for_sort[0..pivot_index].to_string();
    let mut right: String = string_for_sort[pivot_index..string_for_sort.len()].to_string();

    let mut merged: String = String::new();
    let return_string: String = String::new();
    if string_for_sort.len() <= 1 {
        return string_for_sort;
    }

    left = merge_sort(left);
    right = merge_sort(right);

    while left.is_empty() == false && right.is_empty() == false {
        let (left_pop, right_pop) = (left.chars().next().unwrap(), right.chars().next().unwrap());

        if interactive_query(left_pop, right_pop) == '<' {
            merged.push(left_pop);
            left.remove(0);
        } else {
            merged.push(right_pop);
            right.remove(0);
        }
    }

    if left.is_empty() == false {
        merged = merged + &left;
    }
    if right.is_empty() == false {
        merged = merged + &right;
    }
    return merged;
}

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
