use std::result::Result;

pub trait Visitor<T, E> {
    fn visit(&mut self, data:&T) -> Result<T, E>;
}

pub trait VisitorAcceptor<T, E> {
    fn accept<V>(&mut self, visitor:&mut V) where V : Visitor<T, E>;
}

#[cfg(test)]
mod tests {
    use data_structures::linked_list::LinkedList;
    use super::{Visitor, VisitorAcceptor};
    use std::fmt::Debug;

    struct AppendVisitor<T> {
        history: Vec<T>
    }

    impl<T, E> Visitor<T, E> for AppendVisitor<T> where T : Clone {
        fn visit(&mut self, data:&T) -> Result<T, E>{
            self.history.push(data.clone());
            return Ok( data.clone() );
        }
    }

    impl<T> VisitorAcceptor<T, String> for LinkedList<T> where T : Debug + Clone {
        fn accept<V>(&mut self, visitor:&mut V) where V : Visitor<T, String>{
            for item in self.iter(){
                visitor.visit(&item).unwrap();
            }
        }
    }

    #[test]
    fn utilise_visitor_in_linked_list() {
        let mut list=LinkedList::new();
        for i in 0..10 {
            list.insert_back(&i);
        }

        let mut visitor=AppendVisitor{history:Vec::new()};
        list.accept(&mut visitor);
        assert_eq!(visitor.history, vec![0,1,2,3,4,5,6,7,8,9]);
    }
}