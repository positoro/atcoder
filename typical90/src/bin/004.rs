fn main() {
    let h_w: (usize, usize) = input_tuple_usize();
    let a_u32_vec_vec: Vec<Vec<u32>> = input_u32_vector_vector(h_w.0);
    let mut ans_u32_vec_vec: Vec<Vec<u32>> = vec![vec![0; h_w.1]; h_w.0];
    let mut sum_h_vec_u32: Vec<u32> = vec![0; h_w.0];
    let mut sum_w_vec_u32: Vec<u32> = vec![0; h_w.1];

    precalculate_sum(&a_u32_vec_vec, &h_w, &mut sum_h_vec_u32, &mut sum_w_vec_u32);

    for h in 0..h_w.0 {
        for w in 0..h_w.1 {
            ans_u32_vec_vec[h][w] = sum_h_vec_u32[h] + sum_w_vec_u32[w] - a_u32_vec_vec[h][w];
        }
    }
    stdout_vector_vector_u32(&ans_u32_vec_vec);
}

////////////////////////////////////////////////////////////////////////////////

fn precalculate_sum(
    a: &Vec<Vec<u32>>,
    h_w: &(usize, usize),
    return_h_sum: &mut Vec<u32>,
    return_w_sum: &mut Vec<u32>,
) {
    for h in 0..h_w.0 {
        return_h_sum[h] = a[h].iter().sum();
        for w in 0..h_w.1 {
            return_w_sum[w] = return_w_sum[w] + a[h][w];
        }
    }
}

fn stdout_vector_vector_u32(v: &Vec<Vec<u32>>) {
    for h in v.iter() {
        for w in h.iter() {
            print!("{} ", w);
        }
        println!();
    }
}

fn input_tuple_usize() -> (usize, usize) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<usize> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

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
