use std::io;

fn main() {
    
    // how many time we loop
    let n: usize = loop {
        println!("Enter a number");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("failed to convert string to num");
                continue;
            }
        }
    };

    println!("{}", fibonacci(n));

}

fn fibonacci(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;  

    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next
    }

    b
}
