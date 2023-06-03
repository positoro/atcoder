fn main() {
    let input_n_u8: u8 = input_n();
    let mut input_sa_vec: Vec<(String, u32)> = Vec::new();
    input_sa_vec = input_sa(input_n_u8);
    let vec_size = input_sa_vec.len();
    let mut min_index = 0;
    let mut min = input_sa_vec[0].1;

    for i in 0..vec_size {
        if min > input_sa_vec[i].1 {
            min = input_sa_vec[i].1;
            min_index = i;
        }
    }

    for i in 0..vec_size {
        println!("{}", input_sa_vec[(i + min_index) % vec_size].0);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_sa(n: u8) -> Vec<(String, u32)> {
    let mut input_strings = String::new();
    let mut return_vec = Vec::new();

    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<String> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vec.push((v[0].clone(), v[1].parse().unwrap()));
    }

    return return_vec;
}

fn input_n() -> u8 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u8> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
