fn main() {
    let n_u32: u32 = input_u32();
    let a_u32_vec: Vec<u32> = input_u32_vector();
    let mut scores: Vec<(u32, u32)> = Vec::new();

    for i_n in 0..n_u32 {
        let mut counter: u32 = 0;
        for i_a in 0..n_u32 * 3 {
            if a_u32_vec[i_a as usize] == (i_n + 1) {
                counter = counter + 1;
                if counter == 2 {
                    scores.push((i_n, i_a));
                    break;
                }
            }
        }
    }

    scores.sort_by(|a, b| a.1.cmp(&b.1));
    for i in scores.iter() {
        print!("{} ", (i.0 + 1));
    }
}

////////////////////////////////////////////////////////////////////////////////

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
