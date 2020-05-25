fn main() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]; // array literal

    for reference in haystack.iter() {//iter transforma um iterável em iterador
        let item = *reference;        //retorna referencias para os itens, o
                                      //simbolo * dereferencia o endereço;
        let result = match item {   //match checa de o valor "bate" com os
            42 | 132 => "hit!",     //esperados e retorna um valor.
            _        => "miss...",  //o simbolo _ faz o match com qualquer outra
        };                          //opção.

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}

