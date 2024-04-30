use std::io;

// Generate the nth Fibonacci number
fn main() {
    println!("Generate a Nth Fibonacci number!");

    println!("Please input N.");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read input");

    let mut first_value = 0;
    let mut second_value = 1;

    let n: u32 = n.trim().parse().expect("Please type a number");

    if n == 1 {
        println!("Fibonacci number at index {n} is 0")
    } else if n == 2 {
        println!("Fibonacci number at index {n} is 1")
    } else {
        let mut ret = 0;
        for i in 1..n + 1 {
            if i < 3 {
                continue;
            }

            ret = first_value + second_value;
            first_value = second_value;
            second_value = ret;
        }
        println!("Fibonacci number at index {n} is {ret}")
    }
}
