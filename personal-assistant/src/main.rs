use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() {
    greet("Aid", "2019");
    ask_name();
    guess_age();
    prove_count();
    test_knowledge();
    end();
}

fn greet(name: &str, year: &str) {
    println!("Hello! My name is {}.", name);
    println!("I was created in {}.", year);
}

fn ask_name() {
    println!("Please, remind me your name.");
    read_line_with_prompt().map(|name|
        println!("What a great name you have, {}!", name)
    );
}

fn guess_age() {
    println!("Let me guess your age.");
    println!("Enter remainders of dividing your age by 3, 5 and 7.");
    let rem = read_line_with_prompt();
    let vec = rem.iter()
        .flat_map(|s| s.split_whitespace())
        .flat_map(|x| x.parse::<i32>().into_iter())
        .collect::<Vec<i32>>();

    let age = (vec[0] * 70 + vec[1] * 21 + vec[2] * 15) % 105;

    println!("Your age is {}; that's a good time to start programming!", age);
}

fn test_knowledge() {
    println!("Let's test your programming knowledge.");
    println!("Why do we use methods?");
    println!("1. To repeat a statement multiple times.");
    println!("2. To decompose a program into several small subroutines.");
    println!("3. To determine the execution time of a program.");
    println!("4. To interrupt the execution of a program.");
    loop {
        let answer = read_line_with_prompt().unwrap().parse::<i32>().unwrap();
        if answer == 2 {
            break;
        } else {
            println!("Please, try again.");
        }
    }
}

fn read_line_with_prompt() -> Result<String, Box<dyn Error>> {
    print!(">");
    stdout().flush()?;
    let mut name = String::new();
    stdin().read_line(&mut name)?;
    Ok(name.trim().to_string())
}

fn prove_count() {
    println!("Now I will prove to you that I can count to any number you want.");
    let num = read_line_with_prompt().unwrap().parse::<i32>().unwrap();
    for i in 0..num {
        println!("{}!", i);
    }
}

fn end() {
    println!("Congratulations, have a nice day!")
}


