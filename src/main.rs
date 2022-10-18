use std::io;
use dialoguer::Input;

fn main() {
    loop {
        //let mut operator = String::new();

        let first_number: String = Input::new()
                                    .with_prompt("Enter your first number")
                                    .interact()
                                    .expect("Failed to read input");
        let first_number: f32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let operator: String = Input::new()
                            .with_prompt(
                                concat!(
                                    "Choose an operation:\n",
                                    "1 - add\n",
                                    "2 - subtract\n",
                                    "3 - multiply\n",
                                    "4 - divide\n",
                                )
                            )
                            .interact()
                            .expect("Failed to read input");
        let operator: &str = operator.trim();

        let second_number: String = Input::new()
                                    .with_prompt("Enter your second number")
                                    .interact()
                                    .expect("Failed to read input");
        let second_number: f32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if operator == "1" {
            let result = &first_number + &second_number;
            println!("{:.2} + {:.2} = {:.2}", first_number, second_number, result);
        } else if operator == "2" {
            let result = &first_number - &second_number;
            println!("{:.2} - {:.2} = {:.2}", first_number, second_number, result);
        } else if operator == "3" {
            let result = &first_number * &second_number;
            println!("{:.2} * {:.2} = {:.2}", first_number, second_number, result);
        } else if operator == "4" {
            let result = &first_number / &second_number;
            println!("{:.2} / {:.2} = {:.2}", first_number, second_number, result);
        }
    }
}
