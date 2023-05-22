fn main() {
    let (input_h, input_w) = input_h_w();
    let input_string_vector = input_string_vector(input_h);
    let position_of_s = get_position_of_s(input_h, input_w, &input_string_vector);
    let checked_position_of_s = check_s(input_h, input_w, &position_of_s, &input_string_vector);
    let checked_position_of_snuke = check_nuke(&checked_position_of_s, &input_string_vector);

    print_checked_snuke(&checked_position_of_snuke);
}

////////////////////////////////////////////////////////////////////////////////

fn print_checked_snuke(position: &[usize; 3]) {
    let s_h: u32 = position[0] as u32 + 1;
    let s_w: u32 = position[1] as u32 + 1;
    let direction: u32 = position[2] as u32;

    match direction {
        1 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h - 1,
            s_w - 1,
            s_h - 2,
            s_w - 2,
            s_h - 3,
            s_w - 3,
            s_h - 4,
            s_w - 4
        ),
        2 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h - 1,
            s_w,
            s_h - 2,
            s_w,
            s_h - 3,
            s_w,
            s_h - 4,
            s_w
        ),
        3 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h - 1,
            s_w + 1,
            s_h - 2,
            s_w + 2,
            s_h - 3,
            s_w + 3,
            s_h - 4,
            s_w + 4
        ),
        4 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h,
            s_w - 1,
            s_h,
            s_w - 2,
            s_h,
            s_w - 3,
            s_h,
            s_w - 4
        ),
        5 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h,
            s_w + 1,
            s_h,
            s_w + 2,
            s_h,
            s_w + 3,
            s_h,
            s_w + 4
        ),
        6 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h + 1,
            s_w - 1,
            s_h + 2,
            s_w - 2,
            s_h + 3,
            s_w - 3,
            s_h + 4,
            s_w - 4
        ),
        7 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h + 1,
            s_w,
            s_h + 2,
            s_w,
            s_h + 3,
            s_w,
            s_h + 4,
            s_w
        ),
        8 => print!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            s_h,
            s_w,
            s_h + 1,
            s_w + 1,
            s_h + 2,
            s_w + 2,
            s_h + 3,
            s_w + 3,
            s_h + 4,
            s_w + 4
        ),
        _ => (),
    }
}

fn get_position_of_s(h: usize, w: usize, string_vector: &Vec<String>) -> Vec<[usize; 2]> {
    let mut return_vector: Vec<[usize; 2]> = Vec::new();

    for _h in 0..h {
        for _w in 0..w {
            let c: char = get_char_from_string_vector(_h, _w, string_vector);
            if c == 's' {
                let array: [usize; 2] = [_h, _w];
                return_vector.push(array);
            }
        }
    }
    return return_vector;
}

fn check_nuke(positions: &Vec<[usize; 3]>, string_vector: &Vec<String>) -> [usize; 3] {
    let mut return_array: [usize; 3] = [0; 3];

    for pos in positions {
        let s_h: u32 = pos[0] as u32;
        let s_w: u32 = pos[1] as u32;
        let direction: u32 = pos[2] as u32;

        if direction == 1 {
            if get_char_from_string_vector((s_h - 2) as usize, (s_w - 2) as usize, string_vector)
                == 'u'
            {
                if get_char_from_string_vector(
                    (s_h - 3) as usize,
                    (s_w - 3) as usize,
                    string_vector,
                ) == 'k'
                {
                    if get_char_from_string_vector(
                        (s_h - 4) as usize,
                        (s_w - 4) as usize,
                        string_vector,
                    ) == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 1];
                    }
                }
            }
        } else if direction == 2 {
            if get_char_from_string_vector((s_h - 2) as usize, s_w as usize, string_vector) == 'u' {
                if get_char_from_string_vector((s_h - 3) as usize, s_w as usize, string_vector)
                    == 'k'
                {
                    if get_char_from_string_vector((s_h - 4) as usize, s_w as usize, string_vector)
                        == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 2];
                    }
                }
            }
        } else if direction == 3 {
            if get_char_from_string_vector((s_h - 2) as usize, (s_w + 2) as usize, string_vector)
                == 'u'
            {
                if get_char_from_string_vector(
                    (s_h - 3) as usize,
                    (s_w + 3) as usize,
                    string_vector,
                ) == 'k'
                {
                    if get_char_from_string_vector(
                        (s_h - 4) as usize,
                        (s_w + 4) as usize,
                        string_vector,
                    ) == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 3];
                    }
                }
            }
        } else if direction == 4 {
            if get_char_from_string_vector(s_h as usize, (s_w - 2) as usize, string_vector) == 'u' {
                if get_char_from_string_vector(s_h as usize, (s_w - 3) as usize, string_vector)
                    == 'k'
                {
                    if get_char_from_string_vector(s_h as usize, (s_w - 4) as usize, string_vector)
                        == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 4];
                    }
                }
            }
        } else if direction == 5 {
            if get_char_from_string_vector(s_h as usize, (s_w + 2) as usize, string_vector) == 'u' {
                if get_char_from_string_vector(s_h as usize, (s_w + 3) as usize, string_vector)
                    == 'k'
                {
                    if get_char_from_string_vector(s_h as usize, (s_w + 4) as usize, string_vector)
                        == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 5];
                    }
                }
            }
        } else if direction == 6 {
            if get_char_from_string_vector((s_h + 2) as usize, (s_w - 2) as usize, string_vector)
                == 'u'
            {
                if get_char_from_string_vector(
                    (s_h + 3) as usize,
                    (s_w - 3) as usize,
                    string_vector,
                ) == 'k'
                {
                    if get_char_from_string_vector(
                        (s_h + 4) as usize,
                        (s_w - 4) as usize,
                        string_vector,
                    ) == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 6];
                    }
                }
            }
        } else if direction == 7 {
            if get_char_from_string_vector((s_h + 2) as usize, s_w as usize, string_vector) == 'u' {
                if get_char_from_string_vector((s_h + 3) as usize, s_w as usize, string_vector)
                    == 'k'
                {
                    if get_char_from_string_vector((s_h + 4) as usize, s_w as usize, string_vector)
                        == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 7];
                    }
                }
            }
        } else if direction == 8 {
            if get_char_from_string_vector((s_h + 2) as usize, (s_w + 2) as usize, string_vector)
                == 'u'
            {
                if get_char_from_string_vector(
                    (s_h + 3) as usize,
                    (s_w + 3) as usize,
                    string_vector,
                ) == 'k'
                {
                    if get_char_from_string_vector(
                        (s_h + 4) as usize,
                        (s_w + 4) as usize,
                        string_vector,
                    ) == 'e'
                    {
                        return_array = [s_h as usize, s_w as usize, 8];
                    }
                }
            }
        }
    }

    return return_array;
}

fn check_s(
    h: usize,
    w: usize,
    positions: &Vec<[usize; 2]>,
    string_vector: &Vec<String>,
) -> Vec<[usize; 3]> {
    let mut return_vector: Vec<[usize; 3]> = Vec::new();

    for pos in positions {
        let s_h: u32 = pos[0] as u32;
        let s_w: u32 = pos[1] as u32;

        if s_h >= 4
            && s_w >= 4
            && get_char_from_string_vector((s_h - 1) as usize, (s_w - 1) as usize, string_vector)
                == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 1]);
        }
        if s_h >= 4
            && get_char_from_string_vector((s_h - 1) as usize, s_w as usize, string_vector) == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 2]);
        }
        if s_h >= 4
            && (w - 1) as u32 - s_w >= 4
            && get_char_from_string_vector((s_h - 1) as usize, (s_w + 1) as usize, string_vector)
                == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 3]);
        }
        if s_w >= 4
            && get_char_from_string_vector(s_h as usize, (s_w - 1) as usize, string_vector) == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 4]);
        }
        if (w - 1) as u32 - s_w >= 4
            && get_char_from_string_vector(s_h as usize, (s_w + 1) as usize, string_vector) == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 5]);
        }
        if (h - 1) as u32 - s_h >= 4
            && s_w >= 4
            && get_char_from_string_vector((s_h + 1) as usize, (s_w - 1) as usize, string_vector)
                == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 6]);
        }
        if (h - 1) as u32 - s_h >= 4
            && get_char_from_string_vector((s_h + 1) as usize, s_w as usize, string_vector) == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 7]);
        }
        if (w - 1) as u32 - s_w >= 4
            && (h - 1) as u32 - s_h >= 4
            && get_char_from_string_vector((s_h + 1) as usize, (s_w + 1) as usize, string_vector)
                == 'n'
        {
            return_vector.push([s_h as usize, s_w as usize, 8]);
        }
    }

    return return_vector;
}

fn get_char_from_string_vector(h: usize, w: usize, string_vector: &Vec<String>) -> char {
    let return_char: char = string_vector[h].chars().nth(w).unwrap();
    return return_char;
}

fn input_string_vector(low: usize) -> Vec<String> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<String> = Vec::new();

    for _ in 0..low {
        std::io::stdin().read_line(&mut input_strings).ok();
        let push_low: String = input_strings.trim().parse().ok().unwrap();
        return_vec.push(push_low);
        input_strings.clear();
    }

    return return_vec;
}

fn input_h_w() -> (usize, usize) {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<usize> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return (v[0], v[1]);
}
