use std::io;
fn main(){
    const THIS_IS_CONST: u32 = 34*60;
    let x = 5;
    let f=3.2;
    println!("The value of float is :{f}");
    println!("the value of x is :{x}");
    let x = 6;
    println!("The value of x is :{x}");
    println!("THis is const : {THIS_IS_CONST}");


    println!("Please enter the array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read");

    let index: usize = index.trim().parse().expect("Index enterd was not a number");
    give_element(index);

}
fn give_element(index:usize){
    let a = [1,2,3,4,5,6];
    let array_len = a.len();
    if index < array_len{
        let element = a[index];

        println!("the value of the element at index {index} is: {element}");
    }else{
        println!("The len of array is {array_len} but index is {index}");
    }
}