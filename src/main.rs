use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
   hello_rust();
   ejercicio_1();
   ejercicio_2();
}

fn hello_rust() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
} 

fn ejercicio_1() {
    let x: i32 = 5;
    let _y: i32; //waring disabled with underscore

    assert_eq!(x, 5);
    println!("Success!")
}

//fill the blanks in the code to make it compile
fn ejercicio_2() {
    let mut x = 1; //let __ x = 1;
    x += 2;  //__ += 2; 

    assert_eq!(x, 3);
    println!("Success!");
}