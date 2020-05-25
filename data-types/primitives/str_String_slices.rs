fn main() {
    //let name: &str = "Vitor";
    let name = "Vitor"; //string literal = &str = referencia para slice
/*  +---+---+           //essa string é encravada na memória estática do executavel
stack . | 5 |tamanho    //para apenas leitura. Então a variável vai ser um apelido
    +-|-+---+           //para um espaço em memória na stack que contém um
      |                 //ponteiro para esse espaço em memória e o número de elementos
      |
      +--+  
         |  memória estática préalocada
       +-V-+---+---+---+---+
       | V | i | t | o | R |      * slices são janelas que permitem visualizar
       +---+---+---+---+---+        certos espaços em memória
**/

    //poderia ser  String::from("Vitor");
    let mut name2 = "Vitor".to_string(); //Strings são alocadas num buffer na heap.
/*  +---+---+---+                        //A variável name2 vai ser um apelido para
    | . | 8 | 5 | stack                  //um espaço da stack que contém um ponteiro
    +-|-+-|-+-|-+                        //para o primeiro elemento da string na
      |   +---|-->capacidade             //heap, a capacidade desse espaço e o 
      |       |                          //espaço usado.
      |       +-->tamanho
    +-V-+---+---+---+---+---+---+---+
heap| V | i | t | o | r |   |   |   |
    +---+---+---+---+---+---+---+---+
*/    

    let mut name3 = "Vitor".to_string();
    name3.push_str(" Almeida"); 
    let last_name = &name3[6..];

/*     name3: String    last_name: &str
      [-------------]   [-------]
      +---+----+----+---+---+---+ 
stack | . | 16 | 13 |   | . | 7 |
      +-|-+----+----+---+-|-+---+
        |                 |                 +--> string slice: sempre são
        |                 +-----+           |                  referencias.
        |                       |           |
        |                     [-|----------str------------]
      +-V-+---+---+---+---+---+-V-+---+---+---+---+---+---+---+---+---+
heap  | V | i | t | o | r |   | A | l | m | e | i | d | a |   |   |   |   
      +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
*/

}

