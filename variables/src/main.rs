const MILLION: u32 = 1_000_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("million = {}", MILLION);

    let y = 2;
    let y = y * 3;
    println!("y = {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // // compile error!
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // // overflow
    // let mut num: u8 = 255;
    // num = num + 1;
    // println!("num = {}", num);

    let x = 2.0; // f64
    let y: f32 = 1.5; // f32
    let sum = x + y;
    println!("sum = {}", sum);

    let t: bool = true;
    let f: bool = false;
    let res = !(t & f);
    println!("res = {}", res);

    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Cool characters: {}, {}, {}", c, z, heart_eyed_cat);

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_, y, _z) = tup;
    println!("The value of y is: {}, z is {}", y, tup.2);

    let a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];
    println!("a[0] = {}", a[0]);
}
