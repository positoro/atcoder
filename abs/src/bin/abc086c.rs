#[allow(unused_variables)]
fn main() {
    let input_n: u32 = input_n();
    let input_txy_vec_vec: Vec<Vec<u32>> = input_txy_vec_vec(input_n);
    println!("{:?}--{:?}", input_n, input_txy_vec_vec);
}
////////////////////////////////////////////////////////////////////////////////

fn input_txy_vec_vec(n: u32) -> Vec<Vec<u32>> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<Vec<u32>> = Vec::new();
    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        //        array[0] = v[0];
        //        array[1] = v[1];
        //        array[2] = v[2];
        return_vec.push(v.clone());
        input_strings.clear();
    }
    return return_vec;
}

fn input_n() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
