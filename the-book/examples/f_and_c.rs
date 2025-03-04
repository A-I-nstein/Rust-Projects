use std::io::stdin;

fn get_temperature_input(prompt: &str) -> Result<f32, String> {
    let mut input = String::new();
    println!("{}", prompt);
    stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;

    input
        .trim()
        .parse()
        .map_err(|_| "Unable to parse the temperature".to_string())
}

fn f_to_c() -> Result<(), String> {
    let temp_in_f = get_temperature_input("Enter the temperature in F:")?;
    println!("Temperature in C: {}", (temp_in_f - 32.0) * 5.0 / 9.0);
    Ok(())
}

fn c_to_f() -> Result<(), String> {
    let temp_in_c = get_temperature_input("Enter the temperature in C:")?;
    println!("Temperature in F: {}", temp_in_c * 9.0 / 5.0 + 32.0);
    Ok(())
}

fn main() -> Result<(), String> {
    println!("Temperature Conversion");
    println!("F Fahrenheit to Celsius");
    println!("C Celsius to Fahrenheit");
    println!("Input your option:");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;
    let input = input.trim();

    match input {
        "F" => f_to_c()?,
        "C" => c_to_f()?,
        _ => println!("Invalid input."),
    }
    Ok(())
}
