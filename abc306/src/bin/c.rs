use ascii::AsciiChar::v;

fn main() {
    let n_u32 = input_u32();
    let a_u32_vec = input_u32_vector();
    let mut f_vec = Vec::new();

    for i_n in 0..n_u32 {
        let mut counter: u32 = 0;
        for i_a in 0..n_u32 * 3 {
            if a_u32_vec[i_a as usize] == (i_n + 1) {
                counter = counter + 1;
                if counter == 2 {
                    f_vec.push(i_a as u32);
                    break;
                }
            }
        }
    }
    let idx = argsort(&f_vec);

    for i in idx.into_iter() {
        println!("v[{}] = {}", i, v[i]);
    }
}

////////////////////////////////////////////////////////////////////////////////
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}
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
