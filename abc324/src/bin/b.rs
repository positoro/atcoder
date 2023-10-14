fn main() {
    let mut n_u128: u128 = input_u128();

    while n_u128 % 2 == 0 {
        n_u128 = n_u128 / 2;
    }
    while n_u128 % 3 == 0 {
        n_u128 = n_u128 / 3;
    }

    if n_u128 == 0 || n_u128 == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_u128() -> u128 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u128> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
