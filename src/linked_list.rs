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
        if self.is_empty() {
            self.tail=Some( Rc::new(RefCell::new(Node {
                data: data.clone(),
                next: None,
                prev: None
            }) ) );
            self.head=self.tail.clone();
        } else {
            let mut tail = self.tail.take();
            self.tail = Some( Rc::new(RefCell::new(Node {
                data: data.clone(),
                next: None,
                prev: tail.clone(),
            }) ) );
            tail.as_mut().and_then(|link| { link.borrow_mut().next=self.tail.clone(); Some(link.clone())} );
        }
        self.size+=1;
    }

    pub fn insert_front(&mut self, data:&T) {
        if self.is_empty() {
            self.head=Some( Rc::new(RefCell::new(Node {
                data: data.clone(),
                next: None,
                prev: None
            }) ) );
            self.tail=self.head.clone();
        } else {
            let mut head = self.head.take();
            self.head = Some( Rc::new(RefCell::new(Node {
                data: data.clone(),
                next: head.clone(),
                prev: None
            }) ) );
            head.as_mut().and_then(|link| { link.borrow_mut().prev=self.head.clone(); Some(link.clone())} );
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
        if self.size == 1 {
            let ret_val=self.head.as_ref().and_then(|link| Some(link.borrow().data.clone())).or(None);
            self.head=None;
            self.tail=None;
            return ret_val;
        }
        self.head.take().and_then( |value| {
            self.head =value.borrow().next.clone();
            self.size-=1;
            if self.size==1 {
                self.tail=self.head.clone();
            }
            return Some(value.borrow().data.clone())
        } ).or(None)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 1 {
            let ret_val=self.head.as_ref().and_then(|link| Some(link.borrow().data.clone())).or(None);
            self.head=None;
            self.tail=None;
            return ret_val;
        }
        self.tail.take().and_then( |value| {
            self.tail=value.borrow().prev.clone();
            self.size-=1;
            if self.size==1 {
                self.head=self.tail.clone();
            }
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
    fn it_inserts_back_once() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        assert_eq!(list.back().unwrap(),1);
    }

    #[test]
    fn it_inserts_back_twice() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        assert_eq!(list.back().unwrap(),2);
    }

    #[test]
    fn it_inserts_back_three_times() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        assert_eq!(list.back().unwrap(),3);
    }

    #[test]
    fn it_inserts_front_to_empty_list() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        assert_eq!(list.front().unwrap(),1);
    }

    #[test]
    fn it_inserts_front_twice_to_empty_list() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
    }

    #[test]
    fn it_inserts_front_to_one_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
    }

    #[test]
    fn it_inserts_front_twice_to_one_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
    }

    #[test]
    fn it_inserts_front_to_two_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
    }

    #[test]
    fn it_inserts_front_twice_to_two_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
    }

    #[test]
    fn it_inserts_front_to_three_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
    }

    #[test]
    fn it_inserts_front_twice_to_three_length_list() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
    }

    #[test]
    fn it_inserts_back_once_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_back_twice_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        assert_eq!(list.pop_front().unwrap(),1);
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_back_three_times_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        assert_eq!(list.pop_front().unwrap(),1);
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_to_empty_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        assert_eq!(list.pop_front().unwrap(),1);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_empty_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_front().unwrap(),2);
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_to_one_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&13);
        assert_eq!(list.pop_front().unwrap(),13);
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_one_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_front().unwrap(),2);
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_to_two_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&13);
        assert_eq!(list.pop_front().unwrap(),13);
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_two_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_front().unwrap(),2);
        list.pop_front();
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_to_three_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&13);
        assert_eq!(list.pop_front().unwrap(),13);
        list.pop_front();
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_three_length_list_and_pop_front() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_front().unwrap(),2);
        list.pop_front();
        list.pop_front();
        list.pop_front();
        list.pop_front();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_back_once_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        assert_eq!(list.pop_back().unwrap(), 1);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_back_twice_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        assert_eq!(list.pop_back().unwrap(),2);
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_back_three_times_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        assert_eq!(list.pop_back().unwrap(),3);
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_to_empty_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        assert_eq!(list.pop_back().unwrap(),1);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_empty_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_back().unwrap(),1);
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_to_one_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&13);
        assert_eq!(list.pop_back().unwrap(),1);
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_one_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_back().unwrap(),1);
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_to_two_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&13);
        assert_eq!(list.pop_back().unwrap(),2);
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_two_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_back().unwrap(),2);
        list.pop_back();
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_to_three_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&13);
        assert_eq!(list.pop_back().unwrap(),3);
        list.pop_back();
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_three_length_list_and_pop_back() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_back().unwrap(),3);
        list.pop_back();
        list.pop_back();
        list.pop_back();
        list.pop_back();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn it_inserts_front_twice_to_three_length_list_and_pop_back_and_forth() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.pop_back().unwrap(),3);
        assert_eq!(list.pop_front().unwrap(),2);
        assert_eq!(list.pop_back().unwrap(),2);
        assert_eq!(list.pop_front().unwrap(),1);
        assert_eq!(list.pop_back().unwrap(),1);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn it_inserts_back_once_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        assert_eq!(list.back().unwrap(),1);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),1);
    }

    #[test]
    fn it_inserts_back_twice_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        assert_eq!(list.back().unwrap(),2);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),2);
    }

    #[test]
    fn it_inserts_back_three_times_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        assert_eq!(list.back().unwrap(),3);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),3);
    }

    #[test]
    fn it_inserts_front_to_empty_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        assert_eq!(list.front().unwrap(),1);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),1);
    }

    #[test]
    fn it_inserts_front_twice_to_empty_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),2);
    }

    #[test]
    fn it_inserts_front_to_one_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),2);
    }

    #[test]
    fn it_inserts_front_twice_to_one_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),3);
    }

    #[test]
    fn it_inserts_front_to_two_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),3);
    }

    #[test]
    fn it_inserts_front_twice_to_two_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
        assert_eq!(list.count_front(),4);
    }

    #[test]
    fn it_inserts_front_to_three_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&13);
        assert_eq!(list.front().unwrap(),13);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),4);
    }

    #[test]
    fn it_inserts_front_twice_to_three_length_list_and_count() {
        let mut list=LinkedList::new();
        list.insert_back(&1);
        list.insert_back(&2);
        list.insert_back(&3);
        list.insert_front(&1);
        list.insert_front(&2);
        assert_eq!(list.front().unwrap(),2);
        assert_eq!(list.count_front(),list.count_back());
        assert_eq!(list.count_front(),5);
    }
}