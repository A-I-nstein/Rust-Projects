use std::io::stdin;

fn find_nth_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

fn main() -> Result<(), String> {
    println!("Fibonaaci numbers");
    println!("Enter the value of n:");

    let mut n = String::new();
    stdin()
        .read_line(&mut n)
        .map_err(|_| "Failed to read line".to_string())?;
    let n: u32 = n
        .trim()
        .parse()
        .map_err(|_| "Failed to parse n".to_string())?;

    let result = find_nth_fibonacci(n);
    println!("{}th Fibonacci number is {}", n, result);

    Ok(())
}
