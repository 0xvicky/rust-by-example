#![allow(dead_code)] //Allow the unused code
#[derive(Debug)]
struct Person{
    name:String,
    age:u32
}

struct Point{
    x:u32,
    p:u32
}

fn main() {
    println!("Chapter-3: Custom Types");
     //3.1 Structs
    let name = String::from("Vivek");
    let age = 20;

    let vivek = Person{name,age};
    println!("Person is :{},{}",vivek.name,vivek.age);

    let point = Point{ x:3,p:1};
    println!("The coordinates are:{},{}",point.x,point.p);
}
