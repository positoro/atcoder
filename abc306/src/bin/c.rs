use std::collections::HashMap;

fn main() {
    let n_u32 = input_u32();
    let a_u32_vec = input_u32_vector();
    let mut scores = HashMap::new();

    for i_n in 0..n_u32 {
        let mut counter: u32 = 0;
        for i_a in 0..n_u32 * 3 {
            if a_u32_vec[i_a as usize] == (i_n + 1) {
                counter = counter + 1;
                if counter == 2 {
                    scores.insert(i_n, i_a);
                    break;
                }
            }
        }
    }

    let mut vec: Vec<(&u32, &u32)> = scores.iter().collect();
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut vec2: Vec<i32> = Vec::new();
    for i in vec.iter() {
        vec2.push((i.0 + 1) as i32);
    }
    stdout_i32_vector(&vec2);
}

////////////////////////////////////////////////////////////////////////////////

fn stdout_i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
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
