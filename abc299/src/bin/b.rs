use std::collections::HashMap;

fn main() {
    let (input_n, input_t) = input_n_t();
    let input_vec_c = input_vec();
    let input_vec_r = input_vec();
    let color_on_flag: bool = input_vec_c.iter().find(|e| e == &&input_t).is_some();
    let mut game_color = input_t;
    if color_on_flag == false {
        game_color = input_vec_c[0];
    }
    let mut max_index = 0;
    let mut max = 0;
    for n in 0..input_n {
        if game_color == input_vec_c[n as usize] && max < input_vec_r[n as usize] {
            max = input_vec_r[n as usize];
            max_index = n;
        }
    }

    println!("{:?}", max_index + 1);
}

////////////////////////////////////////////////////////////////////////////////

fn input_n_t() -> (u32, u32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_vec() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
