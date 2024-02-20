fn main() {
    use std::io;
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");
    
    let mut length: u32 = length.trim().parse().expect("Please type a positive number!");
    if length%2==1 {
        println!("0");  
        return;
    }
    

    let even_odd = length%4;
    if even_odd==0 {
        length = (length/4)-1;
    }
    else {
        length = length/4;
    }
    println!("{}", length);

    // let mut l:u32;
    // let mut b:u32;
    // let mut count:u32=0;
    // for i in 1..length {
    //     l = length-i;
    //     b = i;
    // 
    //     if l == b {
    //         continue;
    //     }
    //     else {
    //         count+=1;
    //     }
    // }

    // count = count/2;
    // println!("{}", count);
}
