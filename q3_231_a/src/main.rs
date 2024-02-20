fn main() {
    use std::io;
    let mut n_ques = String::new();
    io::stdin()
        .read_line(&mut n_ques)
        .expect("Failed to read line");

    let n_ques: u32 = n_ques.trim().parse().expect("Please type a number!");
    let mut ques_to_attempt:u32 = 0;
    let mut i:u32 = 0;
    
    while i < n_ques {
    let mut count_of_ones:u32 = 0;
        
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("input");
    let nums = line.trim().split(' ').flat_map(str::parse::<u32>).collect::<Vec<_>>();
    for num in nums {
        if num==1 {
            count_of_ones +=1;
        }
    
        }
     if count_of_ones >=2 { 
            ques_to_attempt+=1    
    }
   
    
    i+=1;
    }
    println!("{}", ques_to_attempt); 
    
    
}
