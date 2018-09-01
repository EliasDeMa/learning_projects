use std::io;
use std::io::Write;

fn main() {
    println!("This is a basic calculator\nit implements (+, -, *, /, %, ^)\nenter exit to exit", );

    loop {
        let mut input = String::new();
        let operators = ["+", "-", "*", "/", "%", "^"];

        print!(">>> ", );
        io::stdout()
            .flush()
            .expect("Unable to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line");

        if input.trim() == "exit" {
            std::process::exit(1);
        }
        for operator in operators.iter() {
            match input.find(operator) {
                Some(_) => {
                    let operands: Vec<&str> = input.trim().split(operator).collect();

                    if operands.len() > 2 {
                        println!("Please use only two numbers");
                        continue
                    } else {
                        let op1 = operands[0]
                            .trim()
                            .parse::<f64>()
                            .unwrap();
                        let op2 = operands[1]
                            .trim()
                            .parse::<f64>()
                            .unwrap();

                        match operator {
                            &"+" => {println!("{}", op1 + op2)},
                            &"-" => println!("{}", op1 - op2),
                            &"*" => println!("{}", op1 * op2),
                            &"/" => println!("{}", op1 / op2),
                            &"%" => println!("{}", op1 % op2),
                            &"^" => println!("{}", op1.powf(op2)),
                            _ => println!("This is not a valid input"),
                        }
                    }
                    break
                },
                None => {
                    continue
                },
            }
        }
    }
}
