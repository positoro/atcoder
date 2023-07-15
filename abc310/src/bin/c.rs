fn main() {
    let u32_n: u32 = input_u32();
    let string_vec_s: Vec<String> = input_string_vector(u32_n as usize);
    let mut kind_number: u32 = 0;
    let mut kind_flag_vec: Vec<bool> = vec![true; u32_n as usize];
    for i in 0..u32_n {
        if kind_flag_vec[i as usize] == true {
            let source_s: &String = &string_vec_s[i as usize];
            let source_rev_s: &String =
                &(string_vec_s[i as usize].chars().rev().collect::<String>());
            for j in 0..u32_n {
                if i != j && kind_flag_vec[j as usize] == true {
                    if source_s == &string_vec_s[j as usize]
                        || source_rev_s == &string_vec_s[j as usize]
                    {
                        kind_flag_vec[j as usize] = false;
                    }
                }
            }
        }
    }
    for i in 0..u32_n {
        if kind_flag_vec[i as usize] == true {
            kind_number = kind_number + 1;
        }
    }
    println!("{:?}", kind_number);
}

////////////////////////////////////////////////////////////////////////////////

fn input_string_vector(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
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
