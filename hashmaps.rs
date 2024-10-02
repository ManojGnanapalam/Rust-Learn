use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();

    users.insert(String::from("Viper"),22);
    users.insert(String::from("naveen"),23);
    users.insert(String::from("keerthi"),24);

    let get_user_age = users.get("Vper");

    match get_user_age{
        Some(age) => println!("age of {}",age),
        None => println!("User name invalide"),
    }

}