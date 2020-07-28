use std::ops::{Add}; //Traz a trait Add para o escopo
//traits são como interfaces para algumas linguagens ou protocolos.

//a função vai ser genérica para tipos que, em suas implementações, definiram
//metodos que obedeçam a trait Add e retornem o mesmo tipo passado como param.
                    //(parametro: tipo) -> tipo de retorno
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j // a ultima expressão num bloco é retornada por padrão
}

fn main() {
    let (a, b) = (1.2, 3.2); // atribuição usando pattern matching
    let (x, y) = (10, 20);

    let c = add(a, b);
    let z = add(x, y);

    println!("{} + {} = {}", a, b, c);
    println!("{} + {} = {}", x, y, z);
}
