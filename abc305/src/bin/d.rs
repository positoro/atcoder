fn main() {
    let n_u32: u32 = input_u32();
    let a_u32_vector: Vec<u32> = input_u32_vector();
    let awake_time_vector: Vec<(u32, u32)> = create_awake_time_vector(&a_u32_vector);
    let q_u32: u32 = input_u32();
    let l_r_u32_tuple_vector: Vec<(u32, u32)> = input_u32_tuple_vector(q_u32);
    let mut awaking_time: u32 = 0;
    for (l, r) in l_r_u32_tuple_vector.iter() {
        awaking_time = count_awaking_time(*l, *r, &awake_time_vector);
        println!("{:?}", awaking_time);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn count_awaking_time(s: u32, e: u32, awake_time_vector: &Vec<(u32, u32)>) -> u32 {
    let mut return_time: u32 = 0;

    for i in s..e {
        if check_awaking(i, awake_time_vector) == false {
            return_time = return_time + 1;
        }
    }

    return return_time;
}

fn check_awaking(n: u32, awake_time_vector: &Vec<(u32, u32)>) -> bool {
    let mut return_bool: bool = false;
    for (awake_start, awake_end) in awake_time_vector.iter() {
        if *awake_start <= n && n < *awake_end {
            return_bool = true;
        }
    }

    return return_bool;
}

fn create_awake_time_vector(a: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut awake_time_vector: Vec<(u32, u32)> = Vec::new();
    for (index, value) in a.iter().enumerate().step_by(2) {
        let end: usize = index + 1;
        if end < a.len() {
            awake_time_vector.push((*value, a[end]))
        }
    }

    return awake_time_vector;
}

fn input_u32_tuple_vector(n: u32) -> Vec<(u32, u32)> {
    let mut input_strings = String::new();
    let mut return_vector = Vec::new();

    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vector.push((v[0], v[1]));
    }

    return return_vector;
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
