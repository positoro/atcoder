use std::cmp::max;
use std::cmp::min;

fn main() {
    let a_b_c_tuple: (u64, u64, u64) = input_tuple();
    let mut gcm: u64 = 0;

    get_greatest_common_divisor(
        a_b_c_tuple.0,
        get_greatest_common_divisor(a_b_c_tuple.1, a_b_c_tuple.2),
    );

    if false {
        gcm = get_recursive_greatest_common_divisor(
            a_b_c_tuple.0,
            get_recursive_greatest_common_divisor(a_b_c_tuple.1, a_b_c_tuple.2),
        );
    } else {
        gcm = get_greatest_common_divisor(
            a_b_c_tuple.0,
            get_greatest_common_divisor(a_b_c_tuple.1, a_b_c_tuple.2),
        );
    }

    let cut_times: u64 =
        (a_b_c_tuple.0 / gcm) - 1 + (a_b_c_tuple.1 / gcm) - 1 + (a_b_c_tuple.2 / gcm) - 1;
    println!("{}", cut_times);
}

////////////////////////////////////////////////////////////////////////////////

fn get_greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while a != 0 && b != 0 {
        let bigger: u64 = max(a, b);
        if bigger == a {
            a = bigger % b;
        } else {
            b = bigger % a;
        }
    }
    return max(a, b);
}

fn get_recursive_greatest_common_divisor(a: u64, b: u64) -> u64 {
    let mut bigger: u64 = max(a, b);
    let mut smaller: u64 = min(a, b);
    if smaller == 0 {
        return bigger;
    }

    return get_recursive_greatest_common_divisor(b, a % b);
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
