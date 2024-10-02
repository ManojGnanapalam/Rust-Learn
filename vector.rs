fn main(){
    let mut vec = Vec::new();

    vec.push(2);
    vec.push(3);
    vec.push(2);
    vec.push(4);
    vec.push(5);
    even_filter(&mut vec);
    println!("{:?}",vec);
   // println!("{:?}",ans);
}

fn even_filter(vec: &mut Vec<i32>){
    //let mut new_vec = Vec::new();
    // for val in vec{
    //     if val%2 == 0{
    //         new_vec.push(*val);
    //     }
    // }
    // return new_vec;
    let mut i = 0;
    while i < vec.len(){
        if vec[i] % 2 != 0{
            vec.remove(i);
        }else{
            i +=1;
        }
    }
}