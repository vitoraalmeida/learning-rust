/* implementação para uma lista persistente. Pode usar partes da lista como
 * componentes de outras

list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

list1 -> A ---+
              |                  Box não pode ser usado, pois elementos podem
              v                  ter mais de um dono nessa lista e box n permite
list2 ------> B -> C -> D        Outras linguagens possuem Garbage collection
              ^                  para gerenciar essa situação, Rust não possui
              |                  GC, mas usa contagem de referencia. Mantem um
              |                  registro de quantos donos estão usando um valor.
list3 -> X ---+                  Se um valor possuir pelo menos um dono, ele não
                                 será descartado.

*/

//traz Rc para o escopo.
//Rc permite que se faça cópias dele e o original só será descartado quando todos
//os derivados forem descartados. Mas só é possivel ter referencias compartilhadas
//do valor interno. O valor interno não pode ser mudado nem movido
use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

//iter é igual ao da lista anterior, pois não consome nem altera a lista
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

//pop e push não faz sentdi
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    //como os elementos da lista não podem ser movidos/alterados, para adicionar
    //um item à lista, constroi-se uma nova com o novo elemento seguido da lista
    //antiga
    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    //tail retorna uma lista a partir do segundo elemento
    pub fn tail(&self) -> List<T> {
        //List { head: self.head.as_ref().map(|node| node.next.clone()) }
        //map espera retornar um Valor, mas temos um Option<Valor>, existe
        //um equivalenta para retornar Options: and_then
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    //retorna uma referencia para o primeiro elemento
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            //take não pode ser usado, pois altera o Node interior (next)
            //cur_link = boxed_node.next.take();
            //se a lista for a ultima que conhece o node em questão, Rc pode dar
            //permissão para mover o valor com try_unwrap()
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break 
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}

