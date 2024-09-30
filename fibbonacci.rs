use std::io;

fn main(){
    println!("Enter the number");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("fail to read");
    let num : u32 = num.trim().parse().expect("Plz enter the number only");

    println!("{}",fib(num));
}

fn fib(n:u32)->u32{
    let mut first = 0;
    let mut second = 1;

    if n==0{
        return first;
    }
    if n==1{
        return second;
    }

    for _ in 0..n-1{
        //println!("{i}");
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}