fn main() {
    let input_n = input_s();
    let input_s = input_s();

    let mut counter_a = 0;
    let mut counter_t = 0;
    let mut counter_a_flag: bool = true;

    for c in input_s.chars() {
        if c == 'A' {
            counter_a = counter_a + 1;
            if counter_a == counter_t {
                counter_a_flag = false;
            }
        } else {
            counter_t = counter_t + 1;
            if counter_a == counter_t {
                counter_a_flag = true;
            }
        }
        //        println!("{:?} {} {} {:?}", c, counter_a, counter_t, counter_a_flag);
    }

    if counter_a > counter_t {
        println!("A");
    } else if counter_t > counter_a {
        println!("T");
    } else {
        if counter_a_flag == true {
            println!("A");
        } else {
            println!("T");
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn input_s() -> String {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    input_strings.trim().parse().ok().unwrap()
}
