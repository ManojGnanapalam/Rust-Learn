fn main(){
    let mut name = String::from("Mag mahi");
    name.push_str("Man");
    println!("name is {}",name);
    let ans = first_word(&name);
    println!("ans is {}",ans);

}

fn first_word(str: &String) -> &str {
    let mut space_index=0;
        for i in str.chars(){
        if i==' '{
            break;
        }
        space_index = space_index +1;
    }
    return &str[0..space_index];
}