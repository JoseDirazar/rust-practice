use std::io;

fn main() {
    println!("Ingrese el número para obtener su Fibonacci:");

    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error al leer la entrada.");

        let number = number.trim();

        if number == "exit" || number == "e" {
            println!("Goodbye!");
            break;
        }
        let number: i32 = match number.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingrese un número válido.");
                continue;
            }
        };

        if number < 0 {
            println!("Por favor, ingrese un número no negativo.");
            continue;
        }

        let result = fibonacci(number);
        println!("Fibonacci({}) = {}", number, result);
    }
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    fibonacci(num - 1) + fibonacci(num - 2)
}
