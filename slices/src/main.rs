fn main() {

    let mut s = String::from("Hello World!");

    let length_of = first_word(&s);
    s.clear();

    println!("{}", length_of);

    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}