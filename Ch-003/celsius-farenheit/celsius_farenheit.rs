use std::io;

fn celsius_to_farenheit(celsius: f64) -> f64{
    1.8 * celsius + 32.0
}

fn farenheit_to_celsius(farenheit: f64) -> f64{
    0.556 * (farenheit - 32.0)
}

fn main(){
    let mut user_input = String::new();
    
    println!("Please choose the value you want to convert and if its Celsius or Farenheit:");
    io::stdin().read_line(&mut user_input);

    let mut parts = user_input.split_whitespace();

    let number: Result<f64, _> = parts
        .next()
        .expect("Expected a number")
        .parse();

    let number = match number {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. The first word should be a number.");
            return;
        }
    };

    let mode = parts.collect::<Vec<&str>>().join(" ");

    if mode == "Celsius"{
        println!("{} Celsius degrees is equivalent to {} Farenheit degrees.", number, format!("{:.2}", celsius_to_farenheit(number)));
    }else if mode == "Farenheit"{
        println!("{} Farenheit degrees is equivalent to {} Celsius degrees.", number, format!("{:.2}", farenheit_to_celsius(number)));
    }else{
        println!("Sorry, this temperature scale is invalid!");
    }

    return;
}