fn main() {
    let n_m_u32_tuple: (u32, u32) = input_u32_tuple();
    let c_u32_vec: Vec<String> = input_string_vector();
    let d_u32_vec: Vec<String> = input_string_vector();
    let p_u32_vec: Vec<u32> = input_u32_vector();
    let mut price: u32 = 0;

    for i in 0..n_m_u32_tuple.0 {
        let mut index: usize = 0;

        for (j, d) in d_u32_vec.iter().enumerate() {
            if *d == c_u32_vec[i as usize] {
                index = j + 1;
                break;
            }
        }
        price = price + p_u32_vec[index];
    }

    println!("{:?}", price);
}

////////////////////////////////////////////////////////////////////////////////

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

fn input_string_vector() -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vector: Vec<String> = Vec::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<&str> = input_strings.split_whitespace().collect();
    for s in v.iter() {
        return_vector.push(s.to_string());
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
