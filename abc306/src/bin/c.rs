fn main() {
    let n_u32: u32 = input_u32();
    let a_u32_vec: Vec<u32> = input_u32_vector();
    let mut counter: Vec<u32> = vec![0; (n_u32 + 1) as usize];
    let mut return_vec: Vec<u32> = Vec::new();

    for i_a in 0..a_u32_vec.len() as usize {
        counter[a_u32_vec[i_a] as usize] = counter[a_u32_vec[i_a] as usize] + 1;
        if counter[a_u32_vec[i_a] as usize] == 2 {
            return_vec.push(a_u32_vec[i_a]);
        }
    }

    for i in return_vec.iter() {
        print!("{} ", i);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn get_second(n: u32, a: &Vec<u32>) -> usize {
    let mut return_u32: usize = 0;
    let left: usize = 0;
    let right: usize = a.len();

    let mut counter: u32 = 0;
    for i_a in left..right {
        if a[i_a] == n {
            counter = counter + 1;
            if counter == 2 {
                return_u32 = i_a + 1;
                break;
            }
        }
    }
    return return_u32;
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
