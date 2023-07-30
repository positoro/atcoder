use std::cmp::max;
fn main() {
    let n_m_tuple: (u32, u32) = input_tuple();
    let mut a_vec_u32: Vec<u32> = input_vector_u32();
    let mut b_vec_u32: Vec<u32> = input_vector_u32();

    a_vec_u32.sort();
    a_vec_u32.reverse();
    b_vec_u32.sort();

    let max_price: u32 = *max(a_vec_u32.first().unwrap(), b_vec_u32.first().unwrap());

    let mut a_persons_vec: Vec<u32> = vec![0; (max_price + 1) as usize];
    let mut a_persons: u32 = 0;
    let mut b_persons_vec: Vec<u32> = vec![0; (max_price + 1) as usize];
    let mut b_persons: u32 = b_vec_u32.len() as u32;

    let mut a: u32 = a_vec_u32.pop().unwrap();
    let mut b: u32 = b_vec_u32.pop().unwrap();
    for i in 1..max_price + 1 {
        while a == i {
            if a_vec_u32.len() > 0 {
                a = a_vec_u32.pop().unwrap();
            } else {
                a = 1000000000 + 1;
            }
            a_persons = a_persons + 1;
        }

        a_persons_vec[i as usize] = a_persons;
    }

    for i in (1..max_price + 1).rev() {
        println!("{:?}", i);
        while b == i {
            if b_vec_u32.len() > 0 {
                b = b_vec_u32.pop().unwrap();
            } else {
                b = 0;
            }
            b_persons = b_persons - 1;
        }
        b_persons_vec[i as usize] = b_persons;
    }

    println!("{:?} {:?}", a_persons_vec, b_persons_vec);
}
////////////////////////////////////////////////////////////////////////////////

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
