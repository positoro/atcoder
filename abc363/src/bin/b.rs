fn main() {
    let ntp_vec_u32: Vec<u32> = input_vector_u32();
    let l_vec_u32: Vec<u32> = input_vector_u32();
    let n_u32: u32 = ntp_vec_u32[0];
    let t_u32: u32 = ntp_vec_u32[1];
    let p_u32: u32 = ntp_vec_u32[2];
    let mut day: u32 = 0;

    while check_hair_longer_person(t_u32, p_u32, &l_vec_u32, &day) == false {
        day = day + 1;
    }

    println!("{}", day);
}

////////////////////////////////////////////////////////////////////////////////

fn check_hair_longer_person(t: u32, p: u32, l_vec: &Vec<u32>, day: &u32) -> bool {
    let mut counter = 0;
    for l in l_vec.iter() {
        if l + day >= t {
            counter = counter + 1;
        }
    }
    if counter >= p {
        return true;
    } else {
        return false;
    }
}

///

fn input_vector_u32() -> Vec<u32> {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v;
}
