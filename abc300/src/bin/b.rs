fn main() {
    let (input_h, input_w) = input_h_w();
    let (input_a_char_vec_vec, input_b_char_vec_vec) = input_char_vec_vec(input_h, input_w);

    let count_of_a = counter(&input_a_char_vec_vec, input_h);
    let count_of_b = counter(&input_b_char_vec_vec, input_h);
    let mut print_str: &str = "No";

    for i in 1..input_h {
        for j in 1..input_w {
            let rotated_vec_vec: Vec<Vec<char>> =
                rotate(&input_a_char_vec_vec, i, j, input_h, input_w);
            if check(&rotated_vec_vec, &input_b_char_vec_vec) == true {
                println!("{:?} --- {:?}", &rotated_vec_vec, &input_b_char_vec_vec);
                print_str = "Yes";
                print_str = "Yes";
            }
        }
    }
    println!("{}", print_str);
}

////////////////////////////////////////////////////////////////////////////////
fn rotate(vec_vec: &Vec<Vec<char>>, i: u16, j: u16, h: u16, w: u16) -> Vec<Vec<char>> {
    let mut rotate_source: Vec<Vec<char>> = vec_vec.clone();

    let mut rotate_height: Vec<Vec<char>> = Vec::new();
    let mut rotate_width: Vec<Vec<char>> = Vec::new();

    for _i in 0..i {
        for _h in 1..h {
            rotate_height.push(rotate_source[_h as usize].clone());
        }
        rotate_height.push(rotate_source[0 as usize].clone());
        rotate_source = rotate_height.clone();
        rotate_height.clear();
    }
    for _j in 0..j {
        for _h in 0..h {
            let mut line: Vec<char> = rotate_source[_h as usize].clone();
            let insert_char: char = line.remove(0);
            line.push(insert_char);
            rotate_width.push(line.clone());
        }
        rotate_source = rotate_width.clone();
        rotate_width.clear();
    }
    return rotate_source;
}

fn check(vec_vec_a: &Vec<Vec<char>>, vec_vec_b: &Vec<Vec<char>>) -> bool {
    let return_flag: bool;

    if vec_vec_a == vec_vec_b {
        return_flag = true;
    } else {
        return_flag = false;
    }

    return return_flag;
}

fn counter(vec_vec: &Vec<Vec<char>>, hight: u16) -> u32 {
    let mut return_count: u32 = 0;

    for _h in 0..hight {
        return_count += vec_vec[_h as usize].iter().filter(|&n| *n == '#').count() as u32;
    }

    return return_count;
}

fn input_char_vec_vec(h: u16, w: u16) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut input_strings = String::new();
    let mut return_a_vec: Vec<Vec<char>> = Vec::new();
    let mut return_b_vec: Vec<Vec<char>> = Vec::new();

    for _h in 0..h {
        std::io::stdin().read_line(&mut input_strings).ok();
        let hight: &str = input_strings.trim();
        let width: Vec<char> = hight.chars().collect();
        return_a_vec.push(width);
        input_strings.clear();
    }
    for _h in 0..h {
        std::io::stdin().read_line(&mut input_strings).ok();
        let hight: &str = input_strings.trim();
        let width: Vec<char> = hight.chars().collect();
        return_b_vec.push(width);
        input_strings.clear();
    }
    return (return_a_vec, return_b_vec);
}

fn input_h_w() -> (u16, u16) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u16> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
