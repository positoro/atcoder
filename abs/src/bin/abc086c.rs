#[allow(unused_variables)]
fn main() {
    let input_n: u32 = input_n();
    let input_txy_vec_vec: Vec<Vec<i32>> = input_txy_vec_vec(input_n);
    let mut previous_time: i32 = 0;
    let mut previous_axis: [i32; 2] = [0, 0];
    let mut ableton: &str = "Yes";

    for n in 0..input_n {
        let next_axis: [i32; 2] = [
            input_txy_vec_vec[n as usize][1],
            input_txy_vec_vec[n as usize][2],
        ];

        let axis_length: [i32; 2] = [
            next_axis[0] - previous_axis[0],
            next_axis[1] - previous_axis[1],
        ];
        previous_axis = next_axis;

        let length: i32 = axis_length[0].abs() + axis_length[1].abs();

        let time = input_txy_vec_vec[n as usize][0] - previous_time;
        previous_time = input_txy_vec_vec[n as usize][0];
        if length <= time && (time % length) == 0 {
            continue;
        } else {
            ableton = "No";
            break;
        }
    }

    println!("{}", ableton);
}
////////////////////////////////////////////////////////////////////////////////

fn input_txy_vec_vec(n: u32) -> Vec<Vec<i32>> {
    let mut input_strings = String::new();
    let mut return_vec: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<i32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        return_vec.push(v.clone());
        input_strings.clear();
    }
    return return_vec;
}

fn input_n() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}
