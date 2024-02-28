use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
   hello_rust();
   ejercicio_1();
   ejercicio_2();

   // Scope
    ejercicio_3();
    ejercicio_4();

    //shadowing
    ejercicio_5();
    ejercicio_6();

    //unused variables
    ejercicio_7();

    //destructuring
    ejercicio_8();

    //destructuring assaigment
    ejercicio_9();
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

// Fix the error below with least amount of modification
fn ejercicio_3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// Fix the error with the use of define_x
fn ejercicio_4() {
    define_x()
}

fn define_x( ) {
    let x = "hello";
    println!("{}, world", x); 
}

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn ejercicio_5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Remove a line in the code to make it compile
fn ejercicio_6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

fn ejercicio_7() {
    let _x = 1; 
}

// Fix the error below with least amount of modification
fn ejercicio_8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

/* 
Destructuring assignments
Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.
 */

 fn ejercicio_9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
} 