use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Welcome to the calculator of universe's brain");
    println!("  ");


    let mut _num1 = String::new();
    let mut _num2 = String::new();

    let mut _operator = String::new();

    print!("What is the first number?:  ");
    read(&mut _num1);

    print!("What is the second number?:  ");
    read(&mut _num2);

    print!("What operation would you like to do? [+ - * /]:  ");
    read(&mut _operator);


    let _num1: f32 = _num1.trim().parse().unwrap();
    let _num2: f32 = _num2.trim().parse().unwrap();

    let _operator: char = _operator.trim().chars().next().unwrap();

    let _operators = String:: from("+-*/");

    if !_operators.contains(_operator) {
        println!("There's no such thing as your operator");
        return;
    }

    let result = match _operator {
        '+' => _num1 + _num2,
        '-' => _num1 - _num2,
        '/' => _num1 / _num2,
        '*' => _num1 * _num2,
        _ => panic!("ERROR, err")
    };

    println!("The result of  {} {} {} is  {}",_num1, _operator, _num2, result);

}
