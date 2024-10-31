use std::io;

fn main() {
    println!("Select what you are converting:");
    println!("a) Celsius -> Fahrenheit");
    println!("b) Fahrenheit -> Celsius");

    const POSSIBLE_CONVERSIONS: [&str; 2] = ["a", "b"];
    let mut type_of_conversion = String::new();

    loop {
        io::stdin()
            .read_line(&mut type_of_conversion)
            .expect("Failed to read line");

        if !POSSIBLE_CONVERSIONS.contains(&&*type_of_conversion.trim()) {
            type_of_conversion = String::new();
            continue;
        }

        break;
    }

    loop {
        println!("Please input the decimal number you're trying to convert.");

        let mut convert_number = String::new();
        io::stdin()
            .read_line(&mut convert_number)
            .expect("Failed to read line");

        let convert_number: f64 = match convert_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You are converting: {convert_number}");

        match type_of_conversion.trim().eq("a") {
            true => {
                // celsius to fahrenheit
                println!(
                    "{} celsius converted to fahrenheit -> {:.3}",
                    convert_number,
                    celsius_to_fahrenheit(&convert_number)
                );
                break;
            }
            false => {
                // fahrenheit to celsius
                println!(
                    "{} fahrenheit converted to celsius -> {:.3}",
                    convert_number,
                    fahrenheit_to_celsius(&convert_number).round()
                );
                break;
            }
        }
    }
}

fn celsius_to_fahrenheit(celsius: &f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
