fn main() {
    let u32_tuple_n_m = input_u32_tuple();
    let u32_vec_vec_p = input_u32_vector_vector(u32_tuple_n_m.0 as usize);
    let mut func_vec_vec: Vec<Vec<bool>> = Vec::new();
    for i in 0..u32_tuple_n_m.0 {
        let ci = u32_vec_vec_p[i as usize][1];
        let mut bool_vec: Vec<bool> = vec![false; u32_tuple_n_m.1 as usize];
        for j in 0..ci {
          bool_vec[j] = 

        }
    }

    println!("{:?}", u32_tuple_n_m);
    println!("{:?}", u32_vec_vec_p);
}

////////////////////////////////////////////////////////////////////////////////

fn input_u32_vector_vector(low: usize) -> Vec<Vec<u32>> {
    let mut input_strings = String::new();
    let mut return_u32_vector_vector = Vec::new();
    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_u32_vector_vector.push(v);
    }

    return return_u32_vector_vector;
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
