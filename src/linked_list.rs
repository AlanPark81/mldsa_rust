use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

type OptionLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
struct Node<T>{
    pub prev:OptionLink<T>,
    pub next:OptionLink<T>,
    pub data:T
}

pub struct ListIterator<T> {
    next_node:OptionLink<T>
}

impl<T> Iterator for ListIterator<T> where T : Clone {
    type Item=T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next_node.take().and_then(|next_node| {
            self.next_node=next_node.borrow().next.clone();
            Some(next_node.borrow().data.clone()) }).or(None)
    }
}

pub struct LinkedList<T> {
    head: OptionLink<T>,
    tail: OptionLink<T>,
    size: usize
}

impl<T> LinkedList<T> where T: Clone + Debug {
    pub fn new() -> LinkedList<T> {
        LinkedList{
            head:None,
            tail:None,
            size:0
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator { next_node: self.head.clone() }
    }

    pub fn empty(&self) -> bool {
        self.head.as_ref().and_then(|_| Some(false)).unwrap_or(true)
    }

    pub fn is_empty(&self) -> bool { self.empty() }

    pub fn count_front(&self) -> usize {
        let mut node=self.head.clone();
        let mut count=0;
        while let Some(link)=node {
            count+=1;
            node=link.borrow().next.clone();
        }
        return count;
    }

    pub fn count_back(&self) -> usize {
        let mut node=self.tail.clone();
        let mut count=0;
        while let Some(link)=node {
            count+=1;
            node=link.borrow().prev.clone();
        }
        return count;
    }

    pub fn insert_back(&mut self, data:&T) {
        let temp=self.tail.take();
        self.tail=Some(Rc::new(
            RefCell::new(Node{
                prev: None,
                next: None,
                data: data.clone()}
            )
        )
        );

        if self.is_empty() {
            self.head=self.tail.clone();
        } else {
            let temp_tail=self.tail.clone().unwrap();
            (*temp_tail).borrow_mut().prev=temp.clone();
            let temp1=temp.unwrap();
            (*temp1).borrow_mut().next =self.tail.clone();
        }
        self.size+=1;
    }

    pub fn insert_front(&mut self, data:&T) {
        let temp=self.head.take();
        self.head=Some(Rc::new(
            RefCell::new(Node{
                prev: None,
                next: None,
                data: data.clone()}
            )
        )
        );

        if self.is_empty() {
            self.tail=self.head.clone();
        } else {
            let temp_tail=self.head.clone().unwrap();
            (*temp_tail).borrow_mut().next=temp.clone();
            let temp1=temp.unwrap();
            (*temp1).borrow_mut().prev =self.head.clone();
        }
        self.size+=1;
    }

    pub fn insert_after(&mut self, iterator:&ListIterator<T>, data:&T) -> Result<(), &str> {
        let before=iterator.next_node.clone().unwrap();
        let node= Some(Rc::new(RefCell::new(Node {
            prev:Some(before.clone()),
            next:before.borrow().next.clone(),
            data:data.clone()
        })));

        let temp=(*before).borrow_mut().next.clone().unwrap();
        (*temp).borrow_mut().prev=node.clone();
        (*before).borrow_mut().next=node;
        self.size+=1;
        Ok(())
    }
    pub fn insert_before(&mut self, iterator:&ListIterator<T>, data:&T) -> Result<(), &str> {
        let after=iterator.next_node.clone().unwrap();
        let node=Some(Rc::new(RefCell::new(Node {
            prev:after.borrow().prev.clone(),
            next:Some(after.clone()),
            data:data.clone()
        })));
        let temp = (*after).borrow_mut().prev.clone().unwrap();
        (*temp).borrow_mut().next=node.clone();
        (*after).borrow_mut().prev=node;
        self.size+=1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then( |value| {
            self.head =value.borrow().next.clone();
            self.size-=1;
            return Some(value.borrow().data.clone())
        } ).or(None)
    }

    pub fn front(&self) -> Result<T, &str> {
        self.head.as_ref().and_then(|head| Some(head.borrow().data.clone()) ).ok_or("No element")
    }

    pub fn back(&self) -> Result<T, &str> {
        self.tail.as_ref().and_then(|tail| Some(tail.borrow().data.clone()) ).ok_or("No element")
    }
}

#[cfg(test)]
mod tests{
    use super::LinkedList;
    #[test]
    fn it_test_linked_list() {
        let mut list=LinkedList::new();
        for i in 0..100 {
            list.insert_back(&i);
        }
        for i in 0..100 {
            list.insert_front(&i);
        }
        let mut iterator=list.iter();
        iterator.next();
        iterator.next();
        iterator.next();
        if list.insert_after(&iterator,&318).is_err(){
            panic!("insert_after fail");
        }
        if list.insert_before(&iterator, &31818).is_err() {
            panic!("insert before fail")
        }

        assert_eq!(list.count_back(), list.count_front());
        assert_eq!(list.front().unwrap(), 99);
        assert_eq!(list.back().unwrap(), 99);
    }
}