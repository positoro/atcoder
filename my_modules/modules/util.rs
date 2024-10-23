////////////////////////////////////////////////////////////////////////////////

pub fn _binary_search_check_satisfaction(vector: &Vec<u32>, index: usize, value: u32) -> bool {
    if value <= vector[index] {
        return true;
    } else {
        return false;
    }
}

pub fn binary_search(a: &Vec<u32>, b: u32) -> usize {
    let mut left: i32 = -1;
    let mut right: i32 = a.len() as i32;

    while (left + 1) < right {
        let mid: i32 = (left + right) / 2;
        if _binary_search_check_satisfaction(&a, mid as usize, b) == true {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right as usize;
}

pub fn get_index_of_equal_string_in_string_vector(v_s: &Vec<String>, s: &String) -> i32 {
    let mut return_index: i32 = -1;

    for (j, d) in v_s.iter().enumerate() {
        if *d == *s {
            return_index = j as i32;
            break;
        }
    }
    return return_index;
}

pub fn get_char_from_string_vector(h: usize, w: usize, string_vector: &Vec<String>) -> char {
    let return_char: char = string_vector[h].chars().nth(w).unwrap();
    return return_char;
}
