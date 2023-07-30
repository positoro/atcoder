use std::cmp::max;
fn main() {
    let n_m_tuple: (u32, u32) = input_tuple();
    let mut a_vec_u32: Vec<u32> = input_vector_u32();
    let mut b_vec_u32: Vec<u32> = input_vector_u32();

    a_vec_u32.sort();
    a_vec_u32.reverse();
    b_vec_u32.sort();
    b_vec_u32.reverse();

    let max_price: u32 = *max(a_vec_u32.first().unwrap(), b_vec_u32.first().unwrap());

    let mut a_persons_vec: Vec<u32> = vec![0; (max_price + 1) as usize];
    let mut a_persons: u32 = 0;
    let mut b_persons_vec: Vec<u32> = vec![0; (max_price + 1) as usize];
    let mut b_persons: u32 = 0;

    let mut a: u32 = *a_vec_u32.last().unwrap();
    let mut b: u32 = *b_vec_u32.last().unwrap();
    for i in 1..max_price + 1 {
        print!("i:{:?}", i);
        while a <= i && a_vec_u32.len() > 0 {
            a = a_vec_u32.pop().unwrap();
            println!("a:{:?}", a);
            a_persons = a_persons + 1;
        }
        while b <= i && b_vec_u32.len() > 0 {
            b = b_vec_u32.pop().unwrap();
            b = b_vec_u32.pop().unwrap();
            b_persons = b_persons + 1;
            println!("i:{:?}, persons_b {}", i, b_persons);
        }
        a_persons_vec[i as usize] = a_persons;
        b_persons_vec[i as usize] = b_persons;
    }
    println!("{:?}", a_persons_vec);
    println!("{:?}", b_persons_vec);
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
