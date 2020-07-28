use std::io;
use std::process;

fn main() {
    loop {

        println!("Please enter a first number: ");
        let a = get_user_input();
        
                                                       
        println!("Please enter a second number: ");
        let b = get_user_input();


        let result = sum(a, b);
        println!("{} + {} = {}", a, b, result);
    }
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn get_user_input() -> u32 {
    //String é um tipo de text de tamanho variável que é armazenado na memória heap.
    //String::new() cria um Vector (base para a string) com capacidade 0 e tamanho 0 e retorna o endereço desse Vector.
    //A capacidade desse vector só será definida quando algo for inserido, o que significa que não há alocação até que
    //algo seja inserido.
    let mut input = String::new();
   
    //stdin().read_line() recebe uma referencia mutável para um valor do tipo string pois ira receber o valor
    //do stdin e vai injetar naquele espaço que foi reservado na heap.
    io::stdin().read_line(&mut input).unwrap(); 

    let digit: u32;

    //parse retorna um Result (enum com variantes Ok(value) ou Err(value). Match vai comparar esse retorno com as
    //opções definidas e performa alguma ação.
    match input.trim().parse() {
        Ok(val) => digit = val,  
        Err(err) => {
            println!("This is not a valid number");
            process::exit(1);
        }
    };

    digit
 
}
