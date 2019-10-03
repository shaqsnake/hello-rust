fn main() {
    let s1 = String::from("hello");

    let len = caculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("The value of s is {}", s);

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing  = dangle();
    let borrow_from_something = no_dangle();
}

fn caculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}