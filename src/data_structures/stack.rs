use data_structures::linked_list::LinkedList;

pub struct Stack<T> where T : Clone {
    list: LinkedList<T>
}

pub trait Push<T> where T : Clone {
    fn push(&mut self, data:&T);
}

pub trait Pop<T> where T : Clone {
    fn pop(&mut self) -> Option<T>;
}

impl<T> Stack<T> where T : Clone {
    pub fn new() -> Stack<T> {
        Stack {
            list: LinkedList::<T>::new()
        }
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl<T> Push<T> for Stack<T> where T : Clone {
    fn push(&mut self, data:&T) {
        self.list.insert_back(data);
    }
}

impl<T> Pop<T> for Stack<T> where T : Clone {
    fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}


#[cfg(test)]
mod tests {
    use super::{Stack, Pop, Push};
    #[test]
    fn test_pop_from_empty_stack() {
        let mut stack = Stack::<u32>::new();
        for _ in 0..10 {
            assert!(stack.is_empty());
            assert_eq!(stack.size(), 0);
            assert!(stack.pop().is_none());
        }
    }

    #[test]
    fn test_push_pop_once() {
        let mut stack = Stack::<u32>::new();
        for i in 0..10 {
            stack.push(&i);
            assert!(stack.pop().and_then(|number| Some( number == i ) ).unwrap_or(false));
            assert!(stack.is_empty());
            assert_eq!(stack.size(), 0);
            assert!(stack.pop().is_none());
        }
    }

    #[test]
    fn test_push_pop_twice() {
        let mut stack = Stack::<u32>::new();
        for i in 0..10 {
            stack.push(&i);
            stack.push(&(i+10));
            assert!(stack.pop().and_then(|number| Some(number == (i+10) ) ).unwrap_or(false));
            assert!(stack.pop().and_then(|number| Some(number == i ) ).unwrap_or(false));
            assert!(stack.is_empty());
            assert_eq!(stack.size(), 0);
            assert!(stack.pop().is_none());
        }
    }

}