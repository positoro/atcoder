////////////////////////////////////////////////////////////////////////////////

fn main() {
    let (l, r) = modules::stdin::pair_of_u64_tuple();

    if (l + r) != 1 {
        println!("Invalid");
    } else if l == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

////////////////////////////////////////////////////////////////////////////////

pub mod modules {

    pub mod stdin {

        ////////////////////////////////////////////////////////////////////////////////

        pub fn vector_u32_low(low: &u32) -> Vec<u32> {
            let mut return_vector: Vec<u32> = Vec::new();

            for _ in 0..*low {
                return_vector.push(u32());
            }

            return return_vector;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn u32_vector_vector(low: usize) -> Vec<Vec<u32>> {
            let mut input_strings = String::new();
            let mut return_u32_vector_vector = Vec::new();
            for _ in 0..low {
                std::io::stdin().read_line(&mut input_strings).ok();
                let v: Vec<u32> = input_strings
                    .trim()
                    .split_whitespace()
                    .map(|e| e.parse().ok().unwrap())
                    .collect();
                input_strings.clear();
                return_u32_vector_vector.push(v);
            }

            return return_u32_vector_vector;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn string_vector() -> Vec<String> {
            let mut input_strings = String::new();
            let mut return_vector: Vec<String> = Vec::new();
            std::io::stdin().read_line(&mut input_strings).ok();
            let v: Vec<&str> = input_strings.split_whitespace().collect();
            for s in v.iter() {
                return_vector.push(s.to_string());
            }

            return return_vector;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn u32_tuple_vector(n: u32) -> Vec<(u32, u32)> {
            let mut input_strings = String::new();
            let mut return_vector = Vec::new();

            for _ in 0..(n - 1) {
                std::io::stdin().read_line(&mut input_strings).ok();
                let v: Vec<u32> = input_strings
                    .trim()
                    .split_whitespace()
                    .map(|e| e.parse().ok().unwrap())
                    .collect();
                input_strings.clear();
                return_vector.push((v[0], v[1]));
            }

            return return_vector;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn string_vector_with_low(low: usize) -> Vec<String> {
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

        ////////////////////////////////////////////////////////////////////////////////

        pub fn tuple_low(low: &u64) -> Vec<(u64, u64)> {
            let mut return_vector: Vec<(u64, u64)> = Vec::new();

            for _ in 0..*low {
                return_vector.push(pair_of_u64_tuple());
            }

            return return_vector;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn pair_of_u64_tuple() -> (u64, u64) {
            let mut input_strings = String::new();
            std::io::stdin().read_line(&mut input_strings).ok();
            let v: Vec<u64> = input_strings
                .trim()
                .split_whitespace()
                .map(|e| e.parse().ok().unwrap())
                .collect();
            return (v[0], v[1]);
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn string() -> String {
            let mut input_strings = String::new();
            std::io::stdin().read_line(&mut input_strings).ok();
            input_strings.trim().parse().ok().unwrap()
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn vector_u32() -> Vec<u32> {
            let mut input_strings = String::new();
            std::io::stdin().read_line(&mut input_strings).ok();
            let v: Vec<u32> = input_strings
                .trim()
                .split_whitespace()
                .map(|e| e.parse().ok().unwrap())
                .collect();
            return v;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn u32() -> u32 {
            let mut input_strings = String::new();
            std::io::stdin().read_line(&mut input_strings).ok();
            let v: Vec<u32> = input_strings
                .trim()
                .split_whitespace()
                .map(|e| e.parse().ok().unwrap())
                .collect();
            return v[0];
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    pub mod stdout {

        pub fn vector_vector_u32(v: &Vec<Vec<u32>>) {
            for h in v.iter() {
                for w in h.iter() {
                    print!("{} ", w);
                }
                println!();
            }
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn i32_vector(v: &Vec<i32>) {
            let s_vec: Vec<String> = v.iter().map(|x| x.to_string()).collect();
            println!("{}", s_vec.join(" "));
        }
    }

    ////////////////////////////////////////////////////////////////////////////////

    pub mod util {

        pub fn _binary_search_check_satisfaction(
            vector: &Vec<u32>,
            index: usize,
            value: u32,
        ) -> bool {
            if value <= vector[index] {
                return true;
            } else {
                return false;
            }
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn binary_search(a: &Vec<u32>, b: u32) -> usize {
            let mut left: i32 = -1;
            let mut right: i32 = a.len() as i32;

            while (left + 1) < right {
                let mid: i32 = (left + right) / 2;
                if _binary_search_check_satisfaction(&a, mid as usize, b) == true {
                    right = mid;
                } else {
                    left = mid;
                }
            }
            return right as usize;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn get_index_of_equal_string_in_string_vector(v_s: &Vec<String>, s: &String) -> i32 {
            let mut return_index: i32 = -1;

            for (j, d) in v_s.iter().enumerate() {
                if *d == *s {
                    return_index = j as i32;
                    break;
                }
            }
            return return_index;
        }

        ////////////////////////////////////////////////////////////////////////////////

        pub fn get_char_from_string_vector(
            h: usize,
            w: usize,
            string_vector: &Vec<String>,
        ) -> char {
            let return_char: char = string_vector[h].chars().nth(w).unwrap();
            return return_char;
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
