#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // enum Option<T> { //Already in prelude don't need to be explicit here
    //     Some(T),
    //     None,
    // }
    let some_number = Some(5);
    let some_string = Some("String");

    let absent_number: Option<i32> = None;

    let wallet = Coin::Quarter(UsState::Alaska);

    println!("{:?}", wallet);

    let value = value_in_cents(wallet);

    println!("{}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", none);

    let some_u8_value = 0u8; 
    match some_u8_value { //need to 255 possibilities
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // Placeholder to use as default!
    }
}

