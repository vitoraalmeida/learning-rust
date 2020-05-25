fn main() {
    //arrays em rust são alocados na stack, ou seja, são acessados diretamente
    //não atravbes de ponteiros como em C.
    let one             = [1, 2, 3]; //array literal. Rust infere o tipo
    let two: [u8; 3]    = [1, 2, 3]; //array literal com tipo explicito
    let blank1          = [0; 3]; //inicialização que repete 0, 3 vezes
    let blank2: [u8; 3] = [0; 3];


    let arrays = [one, two, blank1, blank2];

    //Interação com arrays ocorrem atravez de slices.
    //Slices não tem tamanho definido em compilação, pois seu
    //numero de elementos não tem tamanho definido. Então é possivel que se
    //saiba que o retorno vai ser um slice de tipo T, 
    //ainda que nao se saiba o numero de elementos do retorno.

    for a in &arrays { //referenciar um array retorna um slice. Slices suportam
                       //iteração sem usar .iter().
        print!("{:?}: ", a);
        
        let mut sum = 0;
        for i in 0..a.len() { //range
            sum += a[i];
        }

        print!("\t(∑{:?} = {})", a, sum);
        println!("");
    }
}

