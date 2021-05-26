fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    
    let y = {
        let x = 3; //Statements include semicolons
        x + 1    //Expression do not include semicolons(return a value, in this case to y)
    };

    println!("Y is: {}", y);

    let z = five();

    println!("Z is: {}", z);

    //Control Flow:
    let z = 6;

    if z < 6 {
        println!("Z is les than 6");
    } else if z == 6{
        println!("Z is equal to 6");
    } else {
        println!("Z is greater than 6")
    }

    looping();
}

fn another_function(x: i32, y:i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y)
}

fn five() -> i32{
    5
}

fn looping(){
    // loop {
    //     println!("Look me!"); //infinite loop
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Finishing!");
}
//TODO: Convert C° to F° exercise