enum Shape{
    Rectangle(f64,f64),
    Circle(f64),
}

fn main(){
    let rect = Shape::Rectangle(1.2,2.3);
    //calculage_area(rect);
    println!("area of rectangle {}",calculage_area(rect));
    let circle = Shape::Circle(4.5);
    //calculage_area(circle);
    println!("area of circle {}",calculage_area(circle));


}

fn calculage_area(shape: Shape)-> f64{
    match shape{
        Shape::Rectangle(a,b)=>a*b,
        Shape::Circle(r)=>3.14*r*r,
    }
}