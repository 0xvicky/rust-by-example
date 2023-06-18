use std::mem;
use std::fmt;

fn reverse(pair:(i32,&str))->(&str,i32){
let (int_param,string_param) = pair;
return (string_param,int_param);
}

fn main() {
    println!("Chapter-2: Primitives");
    //2.1: LITERALS & OPERATORS
    //Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.

    // integer Addition
    println!("1+2={}",1i32+2);

    // integer subtraction
    // println!("1-2={}",1u32-2);//This will not compile as it runs into overflow
    println!("1-2={}",1i32-2);


    //Short-circuit
    println!("True AND False is:{}", true && false);//false
    println!("True AND False is:{}", true || false);//true
    println!("!True is:{}", !true);//false

    //Bitwise operators
    println!("0011 AND 0101 is {:04b}", 3  & 5);
    println!("One millionsecond is written as {}", 1_000_000u32);//We can add underscore two improve readablitiy

    
    //2.2: TUPLES
    let long_tuple = (1u8, 1u32, -3i8,-8i32);
    println!("First member of the tuple:{}", long_tuple.0);//1
    println!("Second member of the tuple:{}", long_tuple.1);//1
    println!("Third member of the tuple:{}", long_tuple.2);//-3
    println!("Fourth member of the tuple:{}", long_tuple.3);//-8
 
   //We can embed tuples into single tuples
   let tuple_of_tuples = ((4u8,12u32),(-3i32,-8i32));
   println!("Tuple of Tuples:{:?}",tuple_of_tuples);
   //Tuples are printable but more than 12 elements in tuple throws the error
//    let too_long_tuple: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("Too long tuple: {:?}", too_long_tuple);

   //We can also reverse the tuple
   let straight_tuple = (2,"zero-knowledge");
   println!("Reversed Tuple is:{:?}", reverse(straight_tuple));

   //tuple destructing
   let tuple = (false,"zk",1i32,-89i32);
   let (a,b,c,d) = tuple;
   println!("a is:{:?} , b is:{:?}, c is {:?}, d is {:?}", a, b, c, d);

#[derive(Debug)]
   //Structs
   struct Block(
      u32,
      u32,
      String
   );

   let new_block: Block = Block(12142,1,String::from("This is block first"));

println!("New Block:{:?}", new_block);

// println!("Inverted Block:{}",transpose(new_block))

//2.3 Arrays

let arr1:[bool;3] = [true,false,false];//prints normal array
println!("Array 1 is:{:?}",arr1);

let arr2:[bool;500] = [true;500];//prints true 500 times
// println!("Arrray 2 with 500 elements:{:?}",arr2);

//Indexing
println!("last elemtn of array1:{}",arr1[2]);//false

//Length of array
println!("Length of array1:{}",arr1.len());//3

//Arrays are stack allocated, how much bytes occupied
println!("Bytes occupied by array1:{}",mem::size_of_val(&arr1));//3


}
