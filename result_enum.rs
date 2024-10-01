use std::fs::read_to_string;

fn main(){
    let ans = read_from_file(String::from("a.txt"));
    //println!("{}",ans);
}

fn read_from_file(file_path: String)->Result<String, String>{
    let result = read_to_string(file_path);
    match result{
        Ok(data) => Ok(data),
        Err(err) => panic!("File not read"),
    }
}