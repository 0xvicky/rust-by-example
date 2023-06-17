fn main() {
    println!("Chapter-1, Hello World");

    //1.1 Comments

    //Regular Commments
    //First type of comment
    /*Second type of comment */

    //Special comments with 3 backslashes

    //Example
    let x = 5 + /*90 +*/ 5;
    println!("The result of x:{}", x);

     //1.2 Arguments
     //argument
    println!("The number is given to be entered in placeholder:{}",23);
    
    //position argument
    println!("{0}, This is {1}, {1}, This is {0}", "Alice","Bob");//positional argument given by numbers
    //Rust also checks proper number of arguments are given , if you remove bob then it will throw error

    //named arguments
    println!("{subject} {verb} {object}",
    subject="Ram",
    verb="is",
    object="handsome");

    //Format characters in number system
    println!("Base 10: {}", 2);//prints in base 10 i.e as it is
    println!("Base 2: {:b}",2);//Prints in binary
    println!("Base 8: {:o}",2);//Prints in octal
    println!("Base 8: {:o}",289);//Prints in octal
    println!("Base 16: {:x}",29);//Prints in Hexadecimal, bothx have same effects
    println!("Base 16: {:x}",29);//Prints in Hexadecimal, bothx have same effects
    
    let number = 3;
    let width = 11;
    //Specifying the width
    println!("{number:>4}",number =10);//Add spaces on the left of the number
    println!("{number:<4}",number =10);//Add spaces on the right of the number
    println!("{number:$>width$}");//Prints $ 10 times before 3
    println!("{number:$<width$}")//Prints $ 10 times before 3
//the rust can also takes variable like this number or width from surroning variable also

}
