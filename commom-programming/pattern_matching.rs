use std::io;

fn main() {
    println!("Please enter a first number: ");
    
    let mut first = String::new();
    io::stdin().read_line(&mut first); 

    let mut a: u32 = 0;

    match first.trim().parse() { //match permite que o erro seja tratado e o
        Ok(val) => {             //fluxo do programa continue.
            a = val;
        },
        Err(err) => {
            println!("This is not a valid number. Will be considered as 0");
        }
    };
                                                
    println!("Please enter a second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let b: u32 = second.trim().parse().expect("Not a valid number");
    // expect, ao invés de apenas emitir panic, vai retornar a mensagem passada

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}
