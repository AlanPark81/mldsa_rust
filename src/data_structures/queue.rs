use data_structures::linked_list::LinkedList;

pub struct Queue<T> where T : Clone {
    list:LinkedList<T>
}

pub trait Enqueue<T> where T : Clone {
    fn enqueue(&mut self, data:&T);
}

pub trait Dequeue<T> where T : Clone {
    fn dequeue(&mut self) -> Option<T>;
}

impl<T> Enqueue<T> for Queue<T> where T : Clone {
    fn enqueue(&mut self, data:&T) {
        self.list.insert_back(data);
    }
}

impl<T> Dequeue<T> for Queue<T> where T : Clone {
    fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}


impl<T> Queue<T> where T : Clone {
    pub fn new() -> Queue<T> {
        Queue {
            list: LinkedList::<T>::new()
        }
    }

    pub fn empty(&self) -> bool {
        self.list.empty()
    }

    pub fn size(&self) -> usize {
        self.list.size()
    }
}

#[cfg(test)]
mod tests{
    use super::Queue;
    use super::Enqueue;
    use super::Dequeue;

    #[test]
    fn queue_dequeue_for_empty_queue() {
        let mut queue = Queue::<u32>::new();
        assert_eq!(queue.size(), 0);
        assert!(queue.empty());
        assert!(queue.dequeue().is_none());
        for i in 0..10 {
            queue.enqueue(&i);
            queue.enqueue(&(i+10) );
            assert!(queue.dequeue().and_then(|data| Some(data == i)).unwrap_or(false));
            assert!(queue.dequeue().and_then(|data| Some(data == (i+10) ) ).unwrap_or(false));
            assert_eq!(queue.size(), 0);
            assert!(queue.empty());
            assert!(queue.dequeue().is_none());
        }
    }

    #[test]
    fn queue_dequeue_after_one_enqueue() {
        let mut queue = Queue::new();
        for i in 0..10 {
            queue.enqueue(&i);
            assert!(queue.dequeue().and_then(|data| Some(data == i)).unwrap_or(false));
        }
    }

    #[test]
    fn queue_dequeue_after_two_enqueue() {
        let mut queue = Queue::new();
        for i in 0..10 {
            queue.enqueue(&i);
            queue.enqueue(&(i+10) );
            assert!(queue.dequeue().and_then(|data| Some(data == i)).unwrap_or(false));
            assert!(queue.dequeue().and_then(|data| Some(data == (i+10) ) ).unwrap_or(false));
        }
    }

}