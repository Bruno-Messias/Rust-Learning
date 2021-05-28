
fn main() {

    let mut user1 = User {
        email: String::from("someone@hotmail.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    user1.email = String::from("anotheremail@hotmail.com");
    println!("{}", user1.email);

    let email_new = String::from("second@gmail.com");
    let new_username = String::from("thesecond");

    let user2 =  build_user(email_new, new_username);

    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);

    let user3 = User {
        email: String::from("theolderemail@older.com"),
        username: user2.username, //Can use other struct to build a new instance
        ..user1 // uses the rest of user 2 atributes
    };

    println!("{}", user3.email);
    println!("{}", user3.username);
    println!("{}", user3.active);
    println!("{}", user3.sign_in_count);

    //tuples struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("{}", black.0);
    println!("{}", origin.2);

    println!("{:#?}", user1); //Need {:?} ou {:#?}
}

#[derive(Debug)] //Add this to release the print of the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}