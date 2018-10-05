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
        vector.push(data.clone());

        if vector.len() == 1 {
            self.array=vector;
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

    #[test]
    fn it_contains_one_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        assert!(heap.contains(&0));
        assert!(!heap.contains(&1));
    }

    #[test]
    fn it_contains_two_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        heap.push(1);
        assert!(heap.contains(&0));
        assert!(heap.contains(&1));
        assert!(!heap.contains(&2));
    }

    #[test]
    fn it_contains_three_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        heap.push(1);
        heap.push(2);
        assert!(heap.contains(&0));
        assert!(heap.contains(&1));
        assert!(heap.contains(&2));
        assert!(!heap.contains(&3));
    }

    #[test]
    fn it_can_top_empty_heap() {
        let heap: Heap<i32>=Heap::create(Vec::new());
        assert!(heap.top()==None);
    }

    #[test]
    fn it_can_top_one_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        assert!(heap.top().unwrap().clone()==0);
    }

    #[test]
    fn it_can_top_two_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        heap.push(1);
        assert!(heap.top().unwrap().clone()==0);
    }

    #[test]
    fn it_can_top_three_pushed_element() {
        let mut heap=Heap::create(Vec::new());
        heap.push(0);
        heap.push(1);
        heap.push(2);
        assert!(heap.top().unwrap().clone()==0);
    }

    #[test]
    fn it_can_merge_zero_and_one_elements_heap(){
        let mut heap=Heap::create(vec![]);
        let another=Heap::create(vec![1]);
        let expected_seq=vec![1];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_zero_and_two_elements_heap(){
        let mut heap=Heap::create(vec![]);
        let another=Heap::create(vec![1,3]);
        let expected_seq=vec![1,3];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_one_and_one_elements_heap(){
        let mut heap=Heap::create(vec![2]);
        let another=Heap::create(vec![2]);
        let expected_seq=vec![2,2];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_one_and_two_elements_heap(){
        let mut heap=Heap::create(vec![2]);
        let another=Heap::create(vec![1,3]);
        let expected_seq=vec![1,2,3];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_two_and_two_elements_heap(){
        let mut heap=Heap::create(vec![2,6]);
        let another=Heap::create(vec![1,7]);
        let expected_seq=vec![1,2,6,7];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_one_and_three_elements_heap(){
        let mut heap=Heap::create(vec![2]);
        let another=Heap::create(vec![1,3,4]);
        let expected_seq=vec![1,2,3,4];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_two_and_three_elements_heap(){
        let mut heap=Heap::create(vec![2,8]);
        let another=Heap::create(vec![1,3,4]);
        let expected_seq=vec![1,2,3,4,8];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn it_can_merge_three_and_three_elements_heap(){
        let mut heap=Heap::create(vec![2,4,8]);
        let another=Heap::create(vec![1,3,5]);
        let expected_seq=vec![1,2,3,4,5,8];
        heap.merge(&another);

        for i in 0 .. expected_seq.len() {
            assert_eq!(heap.pop().unwrap(), expected_seq[i]);
        }
    }

    #[test]
    fn heap_sort_one_element() {
        let mut to_sort=vec![1];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }

    #[test]
    fn heap_sort_two_elements() {
        let mut to_sort=vec![6,4];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }
    #[test]
    fn heap_sort_three_elements() {
        let mut to_sort=vec![1,6,4];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }
    #[test]
    fn heap_sort_four_elements() {
        let mut to_sort=vec![1,6,4,5];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }
    #[test]
    fn heap_sort_five_elements() {
        let mut to_sort=vec![1,6,4,5,6];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }
    #[test]
    fn heap_sort_six_elements() {
        let mut to_sort=vec![1,6,4,5,6,3];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }

    #[test]
    fn heap_sort_seven_element() {
        let mut to_sort=vec![1,6,4,5,6,3,4];
        let mut expected=to_sort.clone();
        expected.sort();
        let sorted=heap_sort!(to_sort);
        assert_eq!(expected, sorted);
    }

    #[test]
    fn heap_remove_from_zero_elements() {
        let mut heap=Heap::create(vec![]);
        assert!(!heap.contains(&3));
        assert!(!heap.remove(3));
    }

    #[test]
    fn heap_remove_from_one_elements() {
        let mut heap = Heap::create(vec![3]);
        assert!(heap.contains(&3));
        assert!(heap.remove(3));
        assert!(!heap.contains(&3));
    }

    #[test]
    fn heap_remove_from_two_elements() {
        let mut heap = Heap::create(vec![1, 6]);
        assert!(heap.contains(&6));
        assert!(heap.remove(6));
        assert!(!heap.contains(&6));
        assert!(!heap.remove(6));
    }



    #[test]
    fn heap_remove_from_three_elements() {
        let mut heap=Heap::create(vec![1,6,4]);
        assert!(heap.contains(&6));
        assert!(heap.remove(6));
        assert!(!heap.contains(&6));
        assert!(!heap.remove(6));
    }

    #[test]
    fn heap_remove_a_duplicate_element() {
        let mut heap=Heap::create(vec![1,6,4,5,6,3,3]);
        assert!(heap.contains(&6));
        assert!(heap.remove(6));
        assert!(heap.contains(&6));
        assert!(heap.remove(6));
        assert!(!heap.contains(&6));
        assert!(!heap.remove(6));
    }
}
