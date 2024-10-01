// enum NewEnumOption{
//     Some(i32),
//     None,
// }

fn main(){
    let index = find_first_a(String::from("Thousnd"));
    match index{
        Some(i) => println!("The index a present in {}",i),
        None => println!("a is not found"),
    }

}

fn find_first_a(s: String)-> Option<i32>{
    for (index,char) in s.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}