fn main() {
    let s1 = String::from("hello"); //Declre to use the heap
    let s2 = s1; //Move -> invalid the last variable
    let s3 = s2.clone(); //Clone -> do not invalid s2 (its expensive to do)

    println!("{}, world!", s2); //can't print s1-> invalided
    println!("{}, world!", s3); //can print s3-> its valid (but expensive)

    //Ownership:
    let s = String::from("hello");

    println!("{}", s);
    println!("{}", s);
    owner(s);
    //println!("{}",s); //Cant print s anymore(because is passed to the function and dropped)

    let z1 = gives_ownership(); // receiver ownership(in scope)

    let z2 = String::from("hello"); // z2 comes into scope

    let z3 = takes_and_gives_back(z2); //z3 moved, z2 dropped

    println!("{}",z1);
    println!("{}",z3);
    
    //Multiples outs(tuples)
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); //S1 dropped Need to move to s2 to used again

    println!("The length of '{}' is {}.", s2, len);


    //* Pass by reference
    let s1 = String::from("hello");

    let len = calculate_length_pointer(&s1); //Pass by reference the s1 is not dropped(borrow) (u cant modify)

    println!("The length of '{}' is {}.", s1, len);

    //* Mutable references

    let mut s = String::from("hello");

    {let r1 = &mut s;
    //let r2 = &mut s; //! Only can had one mutable reference
    change(r1);
    println!("{}", r1);
    } //r1 is dropped ca have a new mutable reference
    
    let r2 = &mut s;
    println!("{}",r2);


}

fn owner(string_in: String){
    println!("{}",string_in); //Calling the function delete the original string
    //Drop the string
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("hello"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_pointer(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// Don't return a pointer, return the string itself