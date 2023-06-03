fn main() {
    let (n, m) = input_n_m();
    let a = input_u8_vector(m);
    let mut friend_table: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut not_friend_counts: u32 = 0;

    for _m in 0..m {
        for _n in 0..n {
            let s: u8 = get_char_from_vec_vec(_m, _n, m, n, &a);
            set_friend_table(n as usize, _n, s as usize, &a[_m], &mut friend_table);
        }
    }

    not_friend_counts = count_not_friend(&friend_table, n);

    println!("{:?}", not_friend_counts);
}

////////////////////////////////////////////////////////////////////////////////
fn count_not_friend(friend_table: &Vec<Vec<bool>>, n: usize) -> u32 {
    let mut return_count: u32 = 0;

    for i in 0..n {
        for j in 0..n {
            if friend_table[i][j] == false {
                return_count = return_count + 1;
            }
        }
    }

    return_count = (return_count - n as u32) / 2;
    return return_count;
}

fn set_friend_table(
    n_max: usize,
    n: usize,
    person_n: usize,
    line: &Vec<u8>,
    table: &mut Vec<Vec<bool>>,
) {
    if n == 0 {
        table[person_n - 1][(line[1] - 1) as usize] = true;
    } else if n == n_max - 1 {
        table[person_n - 1][(line[n_max - 2] - 1) as usize] = true;
    } else {
        table[person_n - 1][(line[n - 1] - 1) as usize] = true;
        table[person_n - 1][(line[n + 1] - 1) as usize] = true;
    }

    return;
}

fn get_char_from_vec_vec(
    h: usize,
    w: usize,
    m: usize,
    n: usize,
    string_vector: &Vec<Vec<u8>>,
) -> u8 {
    let _h = h % m;
    let _w = w % n;

    let return_u8: u8 = string_vector[_h][_w];
    return return_u8;
}

fn input_u8_vector(low: usize) -> Vec<Vec<u8>> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<Vec<u8>> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();

        let v: Vec<u8> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        return_vec.push(v);
        input_strings.clear();
    }

    return return_vec;
}

fn input_n_m() -> (usize, usize) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<usize> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
fn stdout_i32_vector(v: &Vec<i32>) {
    let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", s_vec.join(" "));
}
