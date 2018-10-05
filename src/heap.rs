struct Heap< T> {
    array:Vec<T>
}

macro_rules! heap_sort{
    ($values:expr) => {{
        for i in 0..$values.len() {
            Heap::heapify(&mut $values[i..]);
        }
        $values
    }}
}

impl<T> Heap<T> where T : Ord + Clone {
    fn heapify(array: &mut [T]){
        for i in 1..array.len() {
            let mut current = i;
            let mut next;
            while current > 0 {
                next=(current+1)/2-1;
                if array[current] < array[next] {
                    array.swap(current, next);
                }
                current=next;
            }
        }
    }
    pub fn create(mut array: Vec<T>) -> Heap<T>{
        if array.is_empty() {
            return Heap {array};
        }

        Heap::heapify(array.as_mut_slice());
        Heap {array}
    }

    pub fn merge(&mut self, another:&Heap<T>){
        for i in 0..another.array.len() {
            self.push(another.array[i].clone());
        }
    }

    pub fn push(&mut self, data: T) {
        let mut vector=self.array.to_vec();
        vector.push(data);

        if vector.len() == 1 {
            return;
        }
        let mut curr_index = self.array.len()-1;
        let mut next_index;
        while curr_index>0 {
            next_index = (curr_index+1)/2-1;
            if vector[next_index] < vector[curr_index] {
                break;
            }
            vector.swap(curr_index, next_index);
            curr_index = next_index;
        }
        self.array=vector;
    }

    pub fn contains(&self, data:&T) -> bool {
        self.array.contains(data)
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.array.is_empty() {
            return None
        }
        let return_val=self.array[0].clone();

        self.array.swap_remove(0);

        if self.array.is_empty(){
            return Some(return_val);
        }
        let mut curr_index=0;
        while (2*curr_index+1)<self.array.len(){
            let next_index;
            let left=curr_index*2+1;
            let right=curr_index*2+2;

            if right==self.array.len() {
                next_index=left;
            } else if self.array[left] < self.array[right] {
                next_index=left;
            } else {
                next_index=right;
            }

            if self.array[next_index] < self.array[curr_index] {
                self.array.swap(next_index, curr_index);
            }
            curr_index=next_index;
        }
        Some(return_val)
    }

    pub fn remove(&mut self, data:T) -> bool {
        let mut index=0;
        let array_len=self.array.len();
        while index<array_len && self.array[index] != data {
            index+=1;
        }
        if index==array_len {
            return false;
        } else {
            let array_len=array_len-1;
            self.array.swap_remove(index);
            let mut left=index*2+1;
            let mut right=index*2+2;
            while ( left<array_len && self.array[index] > self.array[left] ) || ( right < array_len && self.array[index] > self.array[right] ) {
                let next_index;
                if right==array_len || self.array[left] < self.array[right] {
                    next_index = left;
                } else {
                    next_index = right;
                }
                self.array.swap(index, next_index);
                index=next_index;
                left=index*2+1;
                right=index*2+2;
            }
            return true;
        }
    }

    pub fn top(&self) -> Option<&T>{
        if self.is_empty(){
            None
        } else {
            Some(&self.array[0])
        }
    }
}




#[cfg(test)]
mod tests {
    use super::Heap;
    extern crate rand;
    use self::rand::{thread_rng, RngCore};

    #[test]
    fn it_works() {
        let mut heap=Heap::create(Vec::new());
        let mut pushed_seq=Vec::new();
        for _ in 0..100 {
            let data=thread_rng().next_u32() as u8;
            heap.push(data.clone());
            pushed_seq.push(data);
        }
        //pushed_seq.sort_by(|a,b| b.cmp(a));
        pushed_seq.sort();
        pushed_seq.reverse();
        while !heap.is_empty(){
            let top_value=*heap.top().unwrap();
            let popped_value=heap.pop().unwrap();
            assert_eq!(top_value, pushed_seq.pop().unwrap());
            assert_eq!(top_value, popped_value);
        }
    }

    #[test]
    fn merge(){
        let mut heap=Heap::create(vec![1,3,5,7]);
        let another=Heap::create(vec![2,4,6,8]);
        let expected_seq=vec![1,2,3,4,5,6,7,8];
        heap.merge(&another);

        for i in 0 .. 8 {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn heap_sort() {
        let mut to_sort=vec![1,6,4,5,6,3,4,5,2];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }

    #[test]
    fn heap_remove() {
        let mut heap=Heap::create(vec![]);
        assert_eq!(heap.contains(&3), heap.remove(3));
        let mut heap=Heap::create(vec![1,6,4,5,6,3,4,5]);
        assert_eq!(heap.contains(&3), heap.remove(3));
        assert_eq!(heap.contains(&3), heap.remove(3));
        assert_eq!(format!("{:?}", heap.array), "[1, 5, 4, 5, 6, 4, 6]");
        let mut heap=Heap::create(vec![1,6,4,5,6,3]);
        assert_eq!(heap.contains(&3), heap.remove(3));
        assert_eq!(heap.contains(&3), heap.remove(3));

        let mut heap=Heap::create(vec![1,6,4,5,6,3,3]);
        assert_eq!(heap.contains(&3), heap.remove(3));
        assert_eq!(heap.contains(&3), heap.remove(3));
        assert_eq!(heap.contains(&3), heap.remove(3));

        let mut heap=Heap::create(vec![1,6,4,5,8]);
        assert_eq!(heap.contains(&5), heap.remove(5));
    }
}
