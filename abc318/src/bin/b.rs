fn main() {
    let n_u32 = input_u32();
    let mut rect_vec_vec: Vec<Vec<bool>> = vec![vec![false; 101]; 101];
    let tuple_vec_u32 = input_u32_tuple_vector(n_u32);
    let mut counter: u32 = 0;

    for i in 0..n_u32 as usize {
        for x in 0..101 {
            for y in 0..101 {
                if tuple_vec_u32[i].0 <= x
                    && x <= tuple_vec_u32[i].1
                    && tuple_vec_u32[i].2 <= y
                    && y <= tuple_vec_u32[i].3
                {
                    rect_vec_vec[x as usize][y as usize] = true;
                }
            }
        }
    }

    for x in 0..101 as usize {
        for y in 0..101 as usize {
            if rect_vec_vec[x][y] == true {
                counter = counter + 1;
            };
        }
    }
println!("{:?}", counter);
}

////////////////////////////////////////////////////////////////////////////////

fn input_u32_tuple_vector(n: u32) -> Vec<(u32, u32, u32, u32)> {
    let mut input_strings = String::new();
    let mut return_vector = Vec::new();

    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vector.push((v[0], v[1], v[2], v[3]));
    }

    return return_vector;
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
