fn main() {
    let a_b_c_tuple: (u64, u64, u64) = input_tuple();
    let gcm: u64 = get_greatest_common_divisor(a_b_c_tuple.0, a_b_c_tuple.1, a_b_c_tuple.2);
}

////////////////////////////////////////////////////////////////////////////////
fn get_greatest_common_divisor(a: u64, b: u64, c: u64) -> u64 {}

fn _binary_search_check_satisfaction(vector: &Vec<u32>, index: usize, value: u32) -> bool {
    if value <= vector[index] {
        return true;
    } else {
        return false;
    }
}

fn binary_search(a: &Vec<u32>, b: u32) -> usize {
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

fn input_tuple() -> (u64, u64, u64) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u64> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1], v[2]);
}
