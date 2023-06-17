fn main() {
    let a_u32_vec = input_u32_vector();
    let mut return_value: u128 = 0;
    let base: u128 = 2;

    for i in 0..64 {
        if a_u32_vec[i] == 1 {
            return_value = return_value + base.pow(i as u32);
        }
    }

    let idx = argsort(&v);

    for i in idx.into_iter() {
        println!("v[{}] = {}", i, v[i]);
    }
    println!("{:?}", return_value);
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
