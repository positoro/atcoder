fn main() {
    let n_m_tuple_u64: (u64, u64) = input_tuple();
    let l_vec_u64: Vec<u64> = input_vector_u64();
    let mut window_width: u64 = 0;
    let mut window_height: u64 = 0;

    loop {
        window_height = calculate_window_height(window_width, &l_vec_u64);
        println!("{:?}", window_height);

        if window_height != 0 && window_height <= n_m_tuple_u64.1 {
            break;
        }
        window_width = window_width + 1;
    }

    println!("{:?}", window_width);
}

////////////////////////////////////////////////////////////////////////////////

fn calculate_window_height(w: u64, l: &Vec<u64>) -> u64 {
    let mut return_value = 0;
    let mut line_width = 0;

    for u in l.iter() {
        println!("{}", u);
        let buffer_width = w - line_width;
        if buffer_width >= u + 1 {
            line_width = line_width + u + 1;
        } else {
            return_value = return_value + 1;
            line_width = 0;
            line_width = line_width + u;
        }
    }

    return return_value;
}

fn input_tuple() -> (u64, u64) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u64> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_vector_u64() -> Vec<u64> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u64> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
