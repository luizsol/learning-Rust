use std::io;

fn main() {
    let input_is_celsius = loop {
        let mut input_temp_type = String::new();

        println!("What's the type of the temperature to be converted [F|C]?");

        io::stdin().read_line(&mut input_temp_type)
            .expect("Failed to read line");

        let input_temp_type = input_temp_type.trim();

        match input_temp_type.to_lowercase().as_ref() {
            "f" => break false,
            "c" => break true,
            _ => continue,
        }
    };

    let temperature: f64 = loop {
        let mut input_temp_value = String::new();

        println!("Insert the temperature in {}",
                 if input_is_celsius {"°C"} else {"°F"});

        io::stdin().read_line(&mut input_temp_value)
            .expect("Failed to read line");

        match input_temp_value.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    if input_is_celsius {
        println!{"{}°C = {}°F", temperature,
                 celsius_to_fahrenheit(temperature)};
    } else {
        println!{"{}°F = {}°C", temperature,
                 fahrenheit_to_celsius(temperature)};
    }

}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}
