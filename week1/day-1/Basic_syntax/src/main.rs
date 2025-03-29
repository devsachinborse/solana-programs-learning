fn main() {
    println!("Hello, world!");

    
    let  x = 10; //immutabel variable
    println!("x : {:?}", x);

    let mut y = 20; //mutable variable
    println!("y: {:?}", y);

  /*
Integer: i8, i16, i32, i64, i128, isize

Unsigned Integer: u8, u16, u32, u64, u128, usize

Float: f32, f64

Boolean: bool

Character: char

String: String, &str
*/

    let a:i32 = 10;
    let b: f64 = 2.12;
    let c:bool = false;
    let d:char = 'A';

    println!("{}, {},{},{}", a, b,c, d);


    //function
    let result = add(3,4);
    println!("result: {:?}", result);


    //control flow
    let number = 10;
    if number > 5{
        println!("number is greater than 5")
    }else {
        println!("Number is 5 or less")
    }



    //loop
    let mut count = 0;
    loop{
        count += 1;
        println!("Count: {}", count);
        if count == 3{
            break;
        }
    }


    //while loop
    let mut z= 0;
    while z < 5{
        println!("z: {}", z);
        z += 1;
    }


    //iterate over range or collection
        for i in 1..=10 {
            println!("i:{}", i);
        }
    

}




//function 
fn add(x:i32 , y:i32) -> i32 {
    x + y 
}

