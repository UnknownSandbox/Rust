fn main() {
    ownership_as_value();
    ownership_as_reference();
    ownership_as_mut_reference();
    ownership_as_scoped_mut_reference();
    basic_slice_ownerships();
}

fn ownership_as_value() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn ownership_as_reference() {
    let s1 = String::from("hello");
    let len = calculate_length_as_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_as_ref(s: &String) -> usize {
    s.len()
}

fn ownership_as_mut_reference() {
    let mut s = String::from("hello");

    add_world_to_string(&mut s);
    println!("{}", s);
}

fn add_world_to_string(some_string: &mut String) {
    some_string.push_str(", world");
}

fn ownership_as_scoped_mut_reference() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    {
        let r2 = &mut s;
        println!("{}", r2);
    }
}

fn basic_slice_ownerships() {
    let index = first_word(&String::from("hello, Unknown!"));
    println!("{}", index);

    let s = String::from("hello world");
    let index = first_word_sliced(&s);
    println!("{}", index);

    let s = String::from("hello");

    let len = s.len();
    println!("sting length = {}", len);
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
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

fn first_word_sliced(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}