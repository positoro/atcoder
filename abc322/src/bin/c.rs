fn main() {
    let n_m_tuple_u32: (u32, u32) = input_tuple();
    let mut a_vec_u32: Vec<u32> = input_vector_u32();
    a_vec_u32.reverse();

    for i in 1..n_m_tuple_u32.0 + 1 {
        println!("{}", get_fire_day(&mut a_vec_u32, i));
    }
}

////////////////////////////////////////////////////////////////////////////////

fn get_fire_day(a: &mut Vec<u32>, d: u32) -> u32 {
    let mut return_value: u32 = 0;
    if *a.last().unwrap() == d {
        return_value = 0;
        a.pop().unwrap();
    } else {
        return_value = a.last().unwrap() - d;
    }

    return return_value;
}

fn input_tuple() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
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
