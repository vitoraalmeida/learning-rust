//Refatoração de versão anterior usando generics, Option ao invés de criar um
//enum Link, substituição do mem::replace por Option.take() e usa map+closure
//Adiciona método peek que retorna sem remover o primeiro da pilha

struct Node<T> {
    elem: T,
    next: Link<T>,
}

/* Basicamente equivale a Option
 * Options tem um método equivalente a mem::replace
enum Link {
    Empty,
    More(Box<Node>),
}
*/
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

//Iterator que consome a coleção que está sendo iterada
pub struct IntoIter<T>(List<T>);

//Iterator que não consome e só permite leitura. Como não consome, usa referencia.
//Por usar referencia, o lifetime precisa ser valido, pois a estrutura depende
//que a referencia seja valida enquanto a estrutura existir
//O lifetime precisa ser definido explicitamente, pois ,como estamos iterando
//produzindo referencias, a coleção inteira precisa estar válida para que o valor
//da iteração seja válido
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

//não é necessario lifetime, pois List não tem lifetime explicito
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: mem::replace(&mut self.head, Link::Empty),
            next: self.head.take(), //mem::replace para Options
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        /* match option { None => None, Some(x) => Some(y) } é comum e pode ser
         * feito com maps
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        */
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        //pega ownership e não pode-se usar take, pois peek nao remove o item
        //self.head.map(|node| {
        self.head.as_ref().map(|node| {
            //as_ref() pertence a Option e retorna
            &node.elem //                 uma referencia se houver
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            //as_mut() pertence a Option e retorna
            &mut node.elem //           uma referencia mutavel se houver
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //a anotação do método que envolve o tipo com lifetimes precisa conter
    //a definição do lifetime
    //a coleçao precisa ser valida enquanto o iteravel for valido
    //nesse caso específico não é necessário, pois o compilador assume que todos
    //os lifetimes são derivados de self
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        //1)as_ref é necessário pois o map toma a propriedade do node, então
        //o node seria descartado após o fim da função
        //2) as_ref() adiciona indireção, então é preciso derreferenciar
        //3)após dereferenciar, outra indereção é adicionada por Box<Node>,
        //então é preciso dereferenciar antes de ter a referencia para o
        //Node em si
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    //self precisa ser mut, pois dentro da função é usada uma referencia mutavel
    //e não é permitido usar referencia mutavel para uma referencia compartilhada
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //usa pop pois remove o item da coleção e IntoIter consome a coleção
        self.0.pop()
    }
}

//lifetime explicito na implementação, pois Iter possui lifetime explicito
impl<'a, T> Iterator for Iter<'a, T> {
    //lifetime explicito em definições de tipos
    type Item = &'a T;

    //não precisa explicitar o lifetime, pois será inferido atraves do lifetime
    //de self e dos lifitimes da implementação acima
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            //1)as_ref necessário para não tomar a propriedade do node
            //2)uma dereferencia é necessaria pois as_ref adiciona indireção
            //3)após dereferenciar, outra indereção é adicionada por Box<Node>,
            //então é preciso dereferenciar antes de ter a referencia para o
            //Node em si
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        //maps pegam propriedade do valor e tipos que são Copy simplesmente são
        //copiados. self.next para Iter é uma referencia para Node, referencias
        //compartilhadas são copy. Mas next em IterMut é &mut Node, e referencias
        //mutaveis não podem ser copiadas, pois não é permitido ter duas &mut
        //para um mesmo valor. Por isso, o map aqui tenta pegar a propriedade do
        //valor, mas não é permitido mover valores que foram emprestados (& = borrow)
        //por isso, é preciso deixar algo no lugar anterior, assim como no push e pop
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        //|&mut value| não define que o valor é mutavel, apenas é o padrão que
        //se espera. Se bater, ele pega apenas cópia do valor.
        //list.peek_mut().map(|&mut value| {
        list.peek_mut().map(|value| {
            //assim não se pega a copia, mas o valor em si
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

        list.push(4);
        list.push(5);
        list.push(6);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 6));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
