use std::io; //library que lida com entrada e saída de dados
use std::cmp::Ordering; //Traz o enum Ordering para o escopo
use rand::Rng; //Rng é uma trait que define métodos que geradores aleatórios possuem.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess");

        let mut guess = String::new(); //cria uma variavel mutavel do tipo string

        //io::stdin() retorna um handler para entrada e saída
        io::stdin().read_line(&mut guess) //read_line retorna um enum io::Result
            .expect("Failed to read line"); //Reuslts possuem 2 variantes: ok ou err
            //Expect retorna o valor guardado no Result se for Ok(value) e Panic se for Err.

        //match vai comparar o resultado da expressão ao seu lado com as opções
        //passadas dentro do bloco. guess.trim() vai retirar espaços em branco
        //no fim e inicio da string passada pelo usuário. Nesse caso, vai retirar o
        // \n colocado pelo enter. O parse() vai transformar o guess em u32
        // pois rust vai inferir o tipo tendo como referencia o tipo do lado
        //esquero da expressão ( guess: u32). O parse() retorna um result
        // Ok(num) sendo o numero o numero convertido ou Err() se a string não
        //pode ser parseada (caso não tenha sido uma string com um numero)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //se parse retornou ok, retorna o numero
            Err(_) => continue, //se retornou Err, segue o loop
        };
                               //placeholder
        println!("You guessed: {}", guess);

        //guess.cmp retorna um Ordering (enum com 3 variantes) Less, Greater ou Equal, a
        //depender da comparação com o numero passado.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
