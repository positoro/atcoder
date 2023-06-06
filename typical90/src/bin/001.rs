fn main() {
    let input_u32_n_l = input_u32_tuple();
    let input_u32_k = input_u32();
    let input_u32_vec_a = input_u32_vector();
    let mut left: i32 = -1;
    let mut right: i32 = (input_u32_n_l.1 + 1) as i32;

    while right - left > 1 {
        let mut mid: i32 = (right + left) / 2;
        if cut_able_check(
            mid as u32,
            &input_u32_vec_a,
            input_u32_n_l.0,
            input_u32_n_l.1,
            input_u32_k,
        ) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}

////////////////////////////////////////////////////////////////////////////////

fn cut_able_check(score: u32, a: &Vec<u32>, n: u32, l: u32, k: u32) -> bool {
    let mut parts_number: u32 = 0;
    let mut previous_length = 0;
    let mut return_value: bool = false;

    for i in 0..n as usize {
        if a[i] - previous_length >= score {
            parts_number = parts_number + 1;
            previous_length = a[i];
        }
    }
    if l - previous_length >= score {
        parts_number = parts_number + 1;
    }

    if parts_number > k {
        return_value = true;
    } else {
        return_value = false;
    }

    return return_value;
}

fn input_u32_tuple() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_u32_vector() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
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
