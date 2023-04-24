#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    let (input_n, input_q) = input_usizen_u16q();
    let label: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let string_for_sort: String = label[0..input_n].to_string();
    let answer: String;
    if input_n == 5 {
        answer = five_sort(string_for_sort);
    } else {
        //answer = quick_sort(string_for_sort);
        answer = merge_sort(string_for_sort);
    }
    println!("! {}", answer);
}

/////////////////////////////////////////////////////////////////////////////////

fn five_sort(mut string_for_sort: String) -> String {
    let return_string: String = String::new();

    let char_4: char = string_for_sort.pop().unwrap();

    let left: String = string_for_sort[0..2].to_string();
    let right: String = string_for_sort[2..4].to_string();

    let mut merged = merge_sort(left.clone()) + &merge_sort(right.clone());

    let char_of_left_query = merged.chars().nth(1).unwrap();
    let char_of_right_query = merged.chars().nth(3).unwrap();

    if interactive_query(char_of_left_query, char_of_right_query) == '>' {
        let mut merged_swap: String = String::new();

        merged_swap.push(merged.chars().nth(2).unwrap());
        merged_swap.push(merged.chars().nth(0).unwrap());
        merged_swap.push(merged.chars().nth(1).unwrap());
        merged_swap.push(merged.chars().nth(3).unwrap());
        merged = merged_swap.clone();
    } else {
        let mut merged_swap: String = String::new();

        merged_swap.push(merged.chars().nth(0).unwrap());
        merged_swap.push(merged.chars().nth(2).unwrap());
        merged_swap.push(merged.chars().nth(3).unwrap());
        merged_swap.push(merged.chars().nth(1).unwrap());
        merged = merged_swap.clone();
    }

    let char_3: char = merged.pop().unwrap();
    let char_0: char = merged.chars().nth(0).unwrap();

    let mut char_4_inputted: String = binary_search(merged.clone(), char_4);
    let index_char_0: usize = char_4_inputted.find(char_0).unwrap();
    let char_0_second: char = char_4_inputted.remove(index_char_0);
    let mut char_3_inputted: String = binary_search(char_4_inputted.clone(), char_3);
    char_3_inputted.insert(index_char_0, char_0_second);

    return return_string;
}

fn binary_search(mut search_string: String, insert_char: char) -> String {
    if search_string.len() == 0 {
        search_string.push(insert_char);
    } else if search_string.len() == 1 {
        if interactive_query(search_string.chars().nth(0).unwrap(), insert_char) == '>' {
            search_string.insert(0, insert_char);
        } else {
            search_string.push(insert_char);
        }
    } else {
        let m = search_string.len() / 2;
        if interactive_query(search_string.chars().nth(m).unwrap(), insert_char) == '>' {
            search_string = binary_search(search_string[0..m].to_string(), insert_char)
                + &search_string[m..search_string.len()].to_string();
        } else {
            search_string = search_string[m + 1..search_string.len()].to_string()
                + &binary_search(search_string[0..m + 1].to_string(), insert_char);
        }
    }

    return search_string;
}

fn merge_sort(string_for_sort: String) -> String {
    let pivot_index = string_for_sort.len() / 2;
    let mut left: String = string_for_sort[0..pivot_index].to_string();
    let mut right: String = string_for_sort[pivot_index..string_for_sort.len()].to_string();

    let mut merged: String = String::new();
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
