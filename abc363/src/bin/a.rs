mod module;

fn main() {
    let k_u32: u32 = module::input_u32();
    let answer: u32;

    if k_u32 <= 99 {
        answer = 100 - k_u32;
    } else if k_u32 <= 199 {
        answer = 200 - k_u32;
    } else {
        answer = 300 - k_u32;
    }

    println!("{}", answer);
}
