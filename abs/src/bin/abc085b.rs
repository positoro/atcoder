#[allow(unused_variables)]
fn main() {
    let input_n: u16 = input_n();

    let mut input_u16vec_d: Vec<u16> = input_u16vec_d(input_n);
    input_u16vec_d.sort();
    input_u16vec_d.dedup();

    println!("{}", input_u16vec_d.len());
}

////////////////////////////////////////////////////////////////////////////////

fn input_u16vec_d(n: u16) -> Vec<u16> {
    let mut return_vec: Vec<u16> = Vec::new();

    for _ in 0..n {
        let mut input_strings = String::new();
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u16> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        return_vec.push(v[0]);
    }

    return return_vec;
}

fn input_n() -> u16 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
