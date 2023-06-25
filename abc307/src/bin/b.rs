fn main() {
    let n_u32: u32 = input_u32();
    let s_string_vec: Vec<String> = input_string_vector(n_u32 as usize);
    let mut return_flag: bool = false;

    'outer: for i in 0..n_u32 as usize {
        for j in 0..n_u32 as usize {
            let s_string: String = format!("{}{}", s_string_vec[i], s_string_vec[j]);

            let flag: bool = check_palindrome(&s_string);
            if flag == true && i != j {
                return_flag = true;
                break 'outer;
            }
        }
    }

    if return_flag == false {
        println!("No");
    } else {
        println!("Yes");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn check_palindrome(s: &String) -> bool {
    let mut return_flag: bool = true;
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len() - 1 - i).unwrap() {
            return_flag = false;
            break;
        }
    }
    return return_flag;
}

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
