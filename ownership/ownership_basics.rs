//s nao é válido aqui
fn main() {     //s começa a ser válido aqui
    //let s = "hello"; //string literal //hardcoded no programa. Imutavel.
                       //tamanho definido. Colocado na stack
                       // o conteudo é colocado na sua forma bruta, "na mão" no
                       // executável.

    let mut s = String::from("Hello"); // Tipo string. Mutavel. Alocado na heap.
    s.push_str(", world"); // acrescenta no fim um literal
    println!("{}", s);

    let s1 = String::from("Hello"); // strings são alocadas na heap e metadados
    let s2 = s1;                    // são armazenados na stack, como o pontei-
    //aqui, os metadados da string  // para o espaço na heap, o tamanho e capa-
    //são movidos de uma variavel   // cidade
    //para a outra e s1 deixa de ser
    //valida. A string em si continua
    //a mesma na heap. Assim, não há cópia de dados da heap, o que teria um
    //custo maior. String implementam a trait drop que libera o espaço após ter
    //seu valor movido (free no C).

    //println!("{}", s1); //error

    let s1 = String::from("hello");
    let s2 = s1.clone(); //dessa forma os dados da heap são realmente copiados.
                         //uma abordagem mais cara.


    println!("s1 = {}, s2 = {}", s1, s2);


    let x = 5; //valores escalares implementam a trait copy, que permite que os
    let y = x; //valores sejam realmente copiados, pois são armazenados direta-
               //mente na stack, fazendo com que seja uma operação barata.


    println!("y = {}, x = {}",y, x);


    let s1 = String::from("hello");

    takes_ownership(s1); //s1 move seu valor para a  funlção e sai do escopo

    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2); //s2 (originalmente) volta ao escopo
    let x = 5;

    makes_copy(x); //x é i32, que implementa copy, então o valor é copiado e x
                   //pode ser usado posteriormente.




} //s deixa de ser válido

fn takes_ownership(some_string: String) { //some_string entra no escopo
    println!("{}", some_string);
} //some_string sai do escopo e "drop" é chamada.

fn takes_and_give_back(some_string: String) -> String {
    some_string // aqui a função devolve a posse para quem a chama;
}

fn makes_copy(some_integer: i32) { //some_integer entra no escopo
    println!("{}", some_integer);
}//some_iteger sai do escopo, mas nada de mais acontece
