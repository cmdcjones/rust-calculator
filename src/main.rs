use std::io;
use dialoguer::Input;

fn main() {
    loop {
        let first_number = String::new();
        let mut second_number = String::new();
        let mut operator = String::new();

        let first_number: String = Input::new()
                                    .with_prompt("Enter your first number")
                                    .interact()
                                    .expect("Failed to read input");
        let first_number: f32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!(
"Choose an operation:
1 - add
2 - subtract
3 - multiply
4 - divide
> "
        );
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");
        let operator: &str = operator.trim();

        println!("Enter your second number:\n> ");
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");
        let second_number: f32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if operator == "1" {
            let result = &first_number + &second_number;
            println!("{} + {} = {}", first_number, second_number, result);
        } else if operator == "2" {
            let result = &first_number - &second_number;
            println!("{} - {} = {}", first_number, second_number, result);
        } else if operator == "3" {
            let result = &first_number * &second_number;
            println!("{} * {} = {}", first_number, second_number, result);
        } else if operator == "4" {
            let result = &first_number / &second_number;
            println!("{} / {} = {}", first_number, second_number, result);
        }

    }

}
