use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    //panic!("crash and burn");

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    //let mut f = File::open("hello.txt")?; The ? operator

    let _answer = read_username_from_file();

    let _f = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();

    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)

    fs::read_to_string("hello.txt")
    
}
