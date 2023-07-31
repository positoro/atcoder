fn main() {
    let n_m_tuple: (u32, u32) = input_tuple();
    let s_vec: Vec<String> = input_string_vector(n_m_tuple.0 as usize);
    let mut output_vec: Vec<(u32, u32)> = Vec::new();

    for i in 0..n_m_tuple.0 - 8 {
        for j in 0..n_m_tuple.1 - 8 {
            if check(&s_vec, i as usize, j as usize) == true {
                output_vec.push((i + 1, j + 1));
            }
        }
    }
    for o in output_vec.iter() {
        println!("{} {}", o.0, o.1);
    }
}

////////////////////////////////////////////////////////////////////////////////
fn check(s: &Vec<String>, i: usize, j: usize) -> bool {
    let mut return_bool = true;
    //    println!("{}, {} : {:?}", i, j, &s[i][j..j + 4]);
    if &s[i][j..j + 4] != "###." {
        return false;
    } else if &s[i + 1][j..j + 4] != "###." {
        return false;
    } else if &s[i + 2][j..j + 4] != "###." {
        return false;
    } else if &s[i + 3][j..j + 4] != "...." {
        return false;
    } else if &s[i + 5][j + 5..j + 9] != "...." {
        return false;
    } else if &s[i + 6][j + 5..j + 9] != ".###" {
        return false;
    } else if &s[i + 7][j + 5..j + 9] != ".###" {
        return false;
    } else if &s[i + 8][j + 5..j + 9] != ".###" {
        return false;
    }

    return return_bool;
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
