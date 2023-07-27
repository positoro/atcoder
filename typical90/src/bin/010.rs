fn main() {
    let n_u32: u32 = input_u32();
    let c_p_vector_tuple: Vec<(u32, u32)> = input_tuple_low(&n_u32);
    let q_u32: u32 = input_u32();
    let l_r_vector_tuple: Vec<(u32, u32)> = input_tuple_low(&q_u32);

    let mut class_one_cumulative_point_vec: Vec<u32> = vec![0; (n_u32 + 1) as usize];
    let mut class_two_cumulative_point_vec: Vec<u32> = vec![0; (n_u32 + 1) as usize];
    let mut class_one_cumulative_point: u32 = 0;
    let mut class_two_cumulative_point: u32 = 0;

    for i in 1..n_u32 + 1 {
        if c_p_vector_tuple[(i - 1) as usize].0 == 1 {
            class_one_cumulative_point =
                class_one_cumulative_point + c_p_vector_tuple[(i - 1) as usize].1;
        } else {
            class_two_cumulative_point =
                class_two_cumulative_point + c_p_vector_tuple[(i - 1) as usize].1;
        }
        class_one_cumulative_point_vec[i as usize] = class_one_cumulative_point;
        class_two_cumulative_point_vec[i as usize] = class_two_cumulative_point;
    }

    //    println!("{:?}", class_two_cumulative_point_vec);
    for q in 0..q_u32 {
        let class_one_point: u32 = class_one_cumulative_point_vec
            [(l_r_vector_tuple[q as usize].1) as usize]
            - class_one_cumulative_point_vec[(l_r_vector_tuple[q as usize].0 - 1) as usize];
        let class_two_point: u32 = class_two_cumulative_point_vec
            [(l_r_vector_tuple[q as usize].1) as usize]
            - class_two_cumulative_point_vec[(l_r_vector_tuple[q as usize].0 - 1) as usize];

        println!("{} {}", class_one_point, class_two_point);
    }
}

////////////////////////////////////////////////////////////////////////////////

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

fn input_tuple_low(low: &u32) -> Vec<(u32, u32)> {
    let mut return_vector: Vec<(u32, u32)> = Vec::new();

    for _ in 0..*low {
        return_vector.push(input_tuple());
    }

    return return_vector;
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
