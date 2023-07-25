fn main() {
    let n_u32: u32 = input_u32();
    let a_vec_u32: Vec<u32> = input_vector_u32();
    let mut a_sorted: Vec<u32> = a_vec_u32.clone();
    a_sorted.sort();
    let q_u32: u32 = input_u32();
    let b_vec_u32: Vec<u32> = input_vector_u32_low(&q_u32);

    for q in 0..q_u32 {
        println!("{}", binary_search(&a_sorted, b_vec_u32[q as usize]));
    }
}

////////////////////////////////////////////////////////////////////////////////
fn binary_search(a: &Vec<u32>, b: u32) -> u32 {
    let mut return_left_abs_diff = 0;
    let mut return_right_abs_diff = 0;
    let mut return_abs_diff = 0;
    let mut left: usize = 0;
    let mut right: usize = a.len();
    let mut mid: usize = right / 2;

    while (left + 1) != right {
        if b > a[mid] as u32 {
            left = mid;
        } else {
            right = mid;
        }
        mid = (left + right) / 2;
    }

    if a[left] > b {
        return_left_abs_diff = a[left] - b;
    } else {
        return_left_abs_diff = b - a[left];
    }

    if a[right] > b {
        return_right_abs_diff = a[right] - b;
    } else {
        return_right_abs_diff = b - a[right];
    }

    if return_left_abs_diff < return_right_abs_diff {
        return_abs_diff = return_left_abs_diff;
    } else {
        return_abs_diff = return_left_abs_diff;
    }
    return return_abs_diff;
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

fn input_vector_u32() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}

fn input_vector_u32_low(low: &u32) -> Vec<u32> {
    let mut return_vector: Vec<u32> = Vec::new();

    for _ in 0..*low {
        return_vector.push(input_u32());
    }

    return return_vector;
}
