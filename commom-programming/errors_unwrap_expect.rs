use std::io;

fn main() {
    println!("Please enter a first number: ");
    
    let mut first = String::new();
    io::stdin().read_line(&mut first); //é lido como String
    let a: u32 = first.trim().parse().unwrap(); 
    //parse() retorna Result, que é um enum que tem dois tipos:
    //Ok(valor) e Err(). O unwrap vai verificar qual o valor desse result
    //e, se for Ok, desempacota o valor, se for Err, emite panic!
                                                
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
