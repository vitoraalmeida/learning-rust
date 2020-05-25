fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");

    println!("{}", s1);

    change(&mut s1); // como essa referencia sai do escopo quando a função ter-
                     // mina, não tem problema um outro empréstimo de referencia
                     // mutavel

    let r1 = &mut s1;
    //let r2 = &s1; // emprestimos mutaveis e immutaveis da mesma variavel não
                    // podem coexistir

    //let r2 = &mut s1;   //erro, pois uma variavel não pode ser emprestada como
                        //ref mutável mais de uma vez no mesmo contexto. Poderia
                        //gerar data races.

    println!("{}", r1);

    let r2 = &s1; //essa referencia imutavel pode coexistir pois a outra mutavel
                  //já saiu do escopo ao ser usada pelo print. Ou seja, na verdade,
                  //ela não está coexistindo.

    let _r3 = &s1; //varias referencias imutaveis podem coexistir
    let s = return_reference_from_nothing();//a função tenta retornar uma refe
                                            //rencia para um espaco em memoria
                                            //que ja foi descartado pelo fim do
                                            //escopo

}

fn calculate_length(s: &String) -> usize { //passando por referencia,a função
    s.len()                                //não pega a posse do valor
}

fn change(some_string: &mut String) { //para que se possa alterar a string, ela
    some_string.push_str(", world");  //precisa que seja declarada como mutável
                                      //e passada para a função como
                                      //referencia mutavel também.
}

fn return_reference_from_nothing() -> &String { //returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger! Uma referencia para um espaço de memoria que não está sendo
  // mais usado e poderia estar sendo usado por outra coisa no momento.

//para corrigir, deve-se retornar a string em si, não a referencia.
