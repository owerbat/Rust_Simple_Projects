fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let mut x = 5;
    let y = x;
    x += 1;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1);  // compile error

    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("{}, world!", s1);

    let a = [1, 2, 3];
    let _b = a;
    println!("{}", a[0]);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // compile error
    let x = 5;
    makes_copy(x);

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
