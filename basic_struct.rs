struct User{
    first_name:String,
    last_name:String,
    age:u32,
}

impl User{
    fn is_18plus(&self)->bool{
        if self.age > 18{
            return true;
        }else{
            return false;
        }
    }
}

fn main(){
    let user = User{
        first_name:String::from("Viper"),
        last_name:String::from("vaigunth"),
        age:3,
    };
    print!("My name is {} ",user.first_name);
    print!("{}",user.last_name);
    println!(" and i am {} year old",user.age);

    if user.is_18plus()==true{
        println!("He is 18+");
    }else{
        println!("He is not 18+");
    }
    
}