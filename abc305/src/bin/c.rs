fn main() {
    let input_u32_h_w = input_u32_tuple();
    let input_s = input_string_vector(input_u32_h_w.0 as usize);
    let mut max_h = 0;
    let mut min_h = input_u32_h_w.0;
    let mut max_w = 0;
    let mut min_w = input_u32_h_w.1;

    for h in 0..input_u32_h_w.0 {
        for w in 0..input_u32_h_w.1 {
            if get_char_from_string_vector(h as usize, w as usize, &input_s) == '#' {
                if max_h < h {
                    max_h = h;
                }
                if max_w < w {
                    max_w = w;
                }
                if min_h > h {
                    min_h = h;
                }
                if min_w > w {
                    min_w = w;
                }
            }
        }
    }

    let mut return_h_w: (u32, u32) = (0, 0);
    for h in 0..input_u32_h_w.0 {
        for w in 0..input_u32_h_w.1 {
            if max_h >= h && min_h <= h && max_w >= w && min_w <= w {
                if get_char_from_string_vector(h as usize, w as usize, &input_s) == '.' {
                    return_h_w = (h, w);
                }
            }
        }
    }

    println!("{} {}", return_h_w.0 + 1, return_h_w.1 + 1);
}

////////////////////////////////////////////////////////////////////////////////

fn get_char_from_string_vector(h: usize, w: usize, string_vector: &Vec<String>) -> char {
    let return_char: char = string_vector[h].chars().nth(w).unwrap();
    return return_char;
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
fn stdout_i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
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
