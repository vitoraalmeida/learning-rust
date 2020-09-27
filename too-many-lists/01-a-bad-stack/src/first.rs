use std::mem;

/*
pub enum List {
    Empty,
    //Elem(i32, List),
    //definição recursiva não permite saber o tamanho em tempo
    //de compilação. rust precisa saber o tamanho, para isso pode
    //se usar indireção:
    Elem(i32, Box<List>), //box fornece indireção e possui tamanho conhecido
}
stack            heap                        |-----> o variante empty consome
[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)    mais espaço em memória que
                                                     si próprio, pois enums
dividir essa list exige que um elemento seja         precisam ter tamanho que caiba
copiado para a stack e adicionar um Empty no fim     seu maior variante
de cada pedaço.

Gasta mais memória que o necessário para o caso vazio e espaço na stack
*/

/*
   [ptr] -> (Elem A, ptr) -> (Elem B, null)
                                        |não aloca espaço para o caso vazio
*/
struct Node {
    elem: i32,
    next: Link, //quando vazio, não aloca lixo extra
}

//Um link leva a um node ou não existe
//permite alocar espaço para um node apenas se necessário
enum Link {
    //Uma variante não pode ser nula, então a outra variante, como não tem dados
    //associados, pode ser prenchido com zeros - null pointer optimization
    Empty,
    More(Box<Node>), //nao pode ser null,safe rust não possui null pointer
}

//A lista é publica, permitindo seu uso, mas seus componentes são privados,
//deixando os datalhes de implementação escondidos
pub struct List {
    head: Link,
}

//essa implementação é referente a uma pilha
impl List {
    //Self o representa o valor concreto do tipo definido
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    //self -> pega o valor, toma a propriedade, o dono antigo é invalido
    //&mut self -> acesso exclusivo ao valor, contando que deixa válido
    //&self -> permissão de leitura

    //aqui &mut self é necessário para que se possa manejar os valores, trocar
    //suas posiçoes
    pub fn push(&mut self, elem: i32) {
        //o novo elemento da pilha vai no topo, então ele deve apontar para o que
        //antes era a topo
        let new_node = Box::new(Node {
            elem: elem,
            //next: self.head, //resulta em erro, pois move um valor deixando o
            //                 //antigo dono inválido
            //mem::replace entrega o valor que se quer colocando outro no lugar
            //não invalidando pela mudança de propriedade
            next: mem::replace(&mut self.head, Link::Empty),
        });
        //a nova cabeça é o novo valor
        self.head = Link::More(new_node);
    }

    //         &mut - pois pop precisa alterar a lista
    //         Option - pois a lista pode estar vazia e nada ser retirado
    pub fn pop(&mut self) -> Option<i32> {
        //match self.head { //toma a propriedade do valor deixando inválido
        //match &self.head { //não toma propriedade, mas para alterar head, é preciso
        //usar replace para evitar um estado inválido e ter propriedade do valor
        //onde era a cabeça fica vazio, para poder usar o valor atual
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                //a cabeça agora é o proximo do atual
                self.head = node.next;
                //retorna o valor
                Some(node.elem)
            }
        }
        //unimplemented!() //permite não completar a implementação da função
    }
}

//Essa implementação de Drop vai ser usada quando a List sair do escopo e for
//destruida. Será esse o processo usado para destruir a lista.
impl Drop for List {
    fn drop(&mut self) {
        //vai esvaziar o head e pegar o valor dela, com todos os outros
        //nodes seguintes
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        //vai esvaziar cada elemento que existir enquanto houver um próximo
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

//indica para o compilador que todo o modulo de testes só deve ser compilada se
//executando testes
#[cfg(test)]
//cria uma separação entre o código "real" e os testes
mod test {
    //o módulo de teste é um submodulo, para acessar o modulo acima, usar super
    use super::List;

    //indica para a ferramenta de testes que é essa função que se deve executar
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
