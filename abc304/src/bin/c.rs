fn main() {
    let (input_n, input_d): (i32, i32) = input_n_d();
    let input_xy_vec = input_xy_vec(input_n);
    let mut infected_vec: Vec<bool> = vec![false; input_n as usize];

    infected_vec[0] = true;
    for i in 0..input_n as usize {
        if infected_vec[i] == true {
            infect(input_n, input_d, i, &mut infected_vec, &input_xy_vec);
        }
    }

    for i in 0..input_n as usize {
        if infected_vec[i] == true {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn infect(
    n: i32,
    d: i32,
    infect_index: usize,
    infected_vec: &mut Vec<bool>,
    input_xy_vec: &Vec<(i32, i32)>,
) {
    infected_vec[infect_index] = true;

    for i in 0..n as usize {
        for j in 0..n as usize {
            if infected_vec[i] == true {
                if (input_xy_vec[i].0 - input_xy_vec[j].0) * (input_xy_vec[i].0 - input_xy_vec[j].0)
                    + (input_xy_vec[i].1 - input_xy_vec[j].1)
                        * (input_xy_vec[i].1 - input_xy_vec[j].1)
                    <= d * d
                {
                    infected_vec[j] = true;
                    infect(n, d, j, infected_vec, input_xy_vec);
                }
            }
        }
    }
}

fn input_n_d() -> (i32, i32) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<i32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}

fn input_xy_vec(n: i32) -> Vec<(i32, i32)> {
    let mut input_strings = String::new();
    let mut return_vec = Vec::new();

    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<String> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vec.push((v[0].parse().unwrap(), v[1].parse().unwrap()));
    }

    return return_vec;
}
