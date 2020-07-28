use std::ops::{Add}; //Traz a trait Add para o escopo
//traits são como interfaces para algumas linguagens ou protocolos.

//função que funciona especificamente para i32.
//          rebebe dois i32 e retorna i32
fn add_int(a: i32, b: i32) -> i32 {
    //por padrão, serão criadas cópias dos valores passados como argumentos da função
    a + b //ultima expressão do bloco é retornada
}

//a função vai ser genérica para tipos que, em suas implementações, definiram
//metodos que obedeçam a trait Add e retornem o mesmo tipo passado como param.
                    //(parametro: tipo) -> tipo de retorno
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let (a, b) = (1.2, 3.2); // atribuição usando pattern matching
    let (x, y) = (10, 20);

    let w = add_int(x, y);
    let c = add(a, b);
    let z = add(x, y);
    
    println!("{} + {} = {}", x, y, w);
    println!("{} + {} = {}", a, b, c);
    println!("{} + {} = {}", x, y, z);
}
