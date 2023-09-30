fn main() {
    let n_m_tuple_u32: (u32, u32) = input_tuple();
    let s_string = input_s();
    let t_string = input_s();
    let mut output: u32 = 3;
    let find_index: Option<usize> = t_string.find(&s_string);
    let rfind_index: Option<usize> = t_string.rfind(&s_string);

    if find_index.is_none() == false && s_string.is_empty() == false && t_string.is_empty() == false
    {
        if find_index.unwrap() == 0 {
            if rfind_index.unwrap() == t_string.len() - s_string.len() {
                output = 0;
            } else {
                output = 1;
            }
        } else {
            if rfind_index.unwrap() == t_string.len() - s_string.len() {
                output = 2;
            }
        }
    }

    println!("{}", output);
}

////////////////////////////////////////////////////////////////////////////////

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

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}
