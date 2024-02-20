use std::io;
fn reverse(num: String){
    let num_length = num.len();
    // let num1: u32 = num.trim().parse().expect("Not a number");
    for i in 0..num_length{
        let digit = num.as_bytes()[i] as char;
        let digit_int: u64 = digit as u64 - '0' as u64;
        if i == 0 && digit_int == 9 {
            print!("{}", digit_int);
        }
        else if 9-digit_int >= digit_int {
                print!("{}", digit_int);
            }
        else{
            print!("{}", 9-digit_int);
        }
    }
}

fn main() {
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: u64 = num1.trim().parse().expect("NAN");
    reverse(num1.to_string());
}
