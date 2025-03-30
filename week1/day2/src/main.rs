fn main() {
   let s1 = String::from("Hello");
   let s2 = s1; //ownership of s1 is moved to s2

  // println!("s1: {}", s1); // s1 is no longer valid 
   println!("s2: {}", s2); 

   /*
   👉 When s1 is assigned to s2, 
   Rust moves ownership instead of copying, so s1 is no longer accessible.
   */


   let s3 = s2.clone() ;// create a deep copy
   println!("s3:{}", s3);


//borrowing
   
   let s = String::from("Rustlnag");
   print_length(&s) ; //borrowing "s"
   println!("S :{}", s); 
   //👉 s is borrowed (&String), so its ownership remains with main().


   let mut s = String::from("Rust");
   change(&mut s); //mutable borrow
   println!("{}", s) //rustlang 
}


//borrowing
fn print_length(s: &String) {    //s is a referance , no ownership chage
    println!("Length: {}", s.len());
   }

//mutable borrowing
fn change(s: &mut String){
    s.push_str(", programming");
}
