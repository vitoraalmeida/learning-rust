fn main() {
    let name = "Vitor";
    let last = "Almeida".to_string();
    //name = "Ana"; //erro. Variáveis são imutaveis por padrão
    let mut name2 = "Vitor";
    name2 = "Ana";
    println!("{} and {}", name, name2);

    say_name(name.to_string(), last); //o conteudo agora pertence à função
//erro -> println!("{}", last); last não possui mais ao conteudo anterior
    let last = "Almeida".to_string();
    say_name2(&name.to_string(), &last);//passando apenas referencia 
    println!("{}", last);

}

fn say_name(first: String, last: String) {
    println!("Full name: {} {}", first, last);
}
fn say_name2(first: &String, last: &String) { //usando referencias, a função
    println!("Full name: {} {}", first, last);//não pega a propriedade do valor
}
