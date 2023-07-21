fn main() {
    let h_w: (usize, usize) = input_tuple_usize();
    let a_u32_vec_vec: Vec<Vec<u32>> = input_u32_vector_vector(h_w.0);
    let mut ans_u32_vec_vec: Vec<Vec<u32>> = vec![vec![0; h_w.1]; h_w.0];
    let mut sum_h_vec_u32: Vec<u32> = vec![0; h_w.0];
    let mut sum_w_vec_u32: Vec<u32> = vec![0; h_w.1];

    for h in 0..h_w.0 {
        sum_h_vec_u32[h] = sum_h(&a_u32_vec_vec, h);
    }

    for w in 0..h_w.1 {
        sum_w_vec_u32[w] = sum_w(&a_u32_vec_vec, w, &h_w);
    }

    for h in 0..h_w.0 {
        for w in 0..h_w.1 {
            ans_u32_vec_vec[h][w] = sum_h_vec_u32[h] + sum_w_vec_u32[w] - a_u32_vec_vec[h][w];
        }
    }
    stdout_vector_vector_u32(&ans_u32_vec_vec);
}

////////////////////////////////////////////////////////////////////////////////

fn sum_w(a: &Vec<Vec<u32>>, w: usize, h_w: &(usize, usize)) -> u32 {
    let mut return_sum = 0;
    for h in 0..h_w.0 {
        return_sum = return_sum + a[h][w];
    }
    return return_sum;
}

fn sum_h(a: &Vec<Vec<u32>>, h: usize) -> u32 {
    let return_sum = a[h].iter().sum();
    return return_sum;
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
