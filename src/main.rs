use std::io;

fn main() {
    let mut response: String = String::new();

    loop {
        println!("Add, Subtract, Divide or Multiply? (\"exit\" to stop)");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        println!("{response}");

        if response.trim() == "Add" {
            println!("Give 2 numbers to add!");
            let num1 = take_int();
            let num2 = take_int();
            let answ = add(num1, num2);
            println!("Answer: {answ}")
        }

        else if response.trim() == "Subtract" {
            let num1 = take_int();
            let num2 = take_int();
            let answ = subtract(num1, num2);
            println!("Answer: {answ}")
        }

        else if response.trim() == "Divide" {
            let num1 = take_int();
            let num2 = take_int();
            let answ = divide(num1, num2);
            println!("Answer: {answ}")
        }

        else if response.trim() == "Multiply" {
            let num1 = take_int();
            let num2 = take_int();
            let answ = multiply(num1, num2);
            println!("Answer: {answ}")
        }

        else if response.trim() == "exit" {
            break;
        }

        else {
            println!("Not a valid input")
        }
    }
}


fn take_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn subtract(a: usize, b: usize) -> usize {
    a - b
}

fn multiply(a: usize, b: usize) -> usize {
    a * b
}

fn divide(a: usize, b: usize) -> usize {
    if b == 0 {
        println!("Cannot divide by 0!");
        0
    }

    else {
        a / b
    }
}