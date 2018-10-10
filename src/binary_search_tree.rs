use std::boxed::Box;
use std::fmt::Debug;

#[derive(Clone)]
struct Node<T> {
    left: Option< Box< Node<T> > >,
    right: Option< Box< Node<T> > >,
    data: T
}

impl<T> Node<T> where T : Clone + Ord + Debug {

    pub fn max_value(&self) -> T {
        self.right.as_ref().and_then(|right| Some(right.max_value())).unwrap_or(self.data.clone())
    }

    pub fn remove(&mut self, data: &T) -> Option< Box < Node<T> > > {
        if self.data==*data {
            if self.left.is_some() {
                let left=self.left.as_ref().unwrap().clone();
                if left.right.is_none() {
                    self.data=left.data.clone();
                    self.left=left.left.clone();
                    return Some(Box::new(self.clone()))
                } else {
                    let new_data = left.max_value();
                    self.left.as_mut().unwrap().remove(&new_data);
                    self.data=new_data;
                    return Some(Box::new(self.clone()))
                }
            } else if self.right.is_some() {
                self.data=self.right.as_ref().unwrap().data.clone();
                self.left=self.right.as_ref().unwrap().left.clone();
                self.right=self.right.as_ref().unwrap().right.clone();
                return Some(Box::new(self.clone()))
            } else {
                return None;
            }
        } else if self.data<*data && self.right.is_some() {
            self.right=self.right.as_mut().unwrap().remove(data);
            return Some(Box::new(self.clone()))
        } else if self.data>*data && self.left.is_some() {
            self.left=self.left.as_mut().unwrap().remove(data);
            return Some(Box::new(self.clone()))
        } else {
            return None;
        }

    }

    pub fn contains(&self, data: &T )-> bool {
        if self.data==*data {
            return true;
        } else {
            if self.data<*data {
                self.right.as_ref().and_then(|right| Some(right.contains(data))).unwrap_or(false)
            } else{
                self.left.as_ref().and_then(|left| Some(left.contains(data))).unwrap_or(false)
            }
        }
    }

    pub fn insert(&mut self, data: &T) {
        if *data==self.data{
            return
        }
        if *data<self.data {
            let left=self.left.take();
            self.left = left.and_then(
                |mut node| { node.insert( data ); Some(node)})
                .or(Some( Box::new( Node {data: data.clone(), left: None, right: None}) ) );
        } else {
            let right=self.right.take();
            self.right = right.and_then(
                |mut node| { node.insert( data ); Some(node)})
                .or(Some( Box::new( Node {data: data.clone(), left: None, right: None}) ) );
        }
    }
}

pub struct BinarySearchTree<T> {
    root:Option< Box< Node<T> > >
}

impl<T> BinarySearchTree<T> where T : Clone + Ord + Debug {
    pub fn new() ->BinarySearchTree<T> {
        BinarySearchTree{
            root:None
        }
    }

    pub fn create(data: &T) ->BinarySearchTree<T> {
        BinarySearchTree{
            root:Some(Box::new(Node{left: None, right: None, data: data.clone()}))
        }
    }

    pub fn contains(&self, data: &T) -> bool {
        if self.root.is_none() {return false}
        if *data==self.root.as_ref().unwrap().data{
            return true;
        } else if *data < self.root.as_ref().unwrap().data {
            return self.root.as_ref().unwrap().left.as_ref().and_then(|tree| { Some(tree.contains(data))}).unwrap_or(false);
        } else {
            return self.root.as_ref().unwrap().right.as_ref().and_then(|tree| { Some(tree.contains(data))}).unwrap_or(false);
        }
    }

    pub fn insert(&mut self, data: &T) {
        self.root=self.root.as_mut().and_then(|node| { node.insert(data); Some(node.clone()) }).or_else(|| Some(Box::new(Node{data:data.clone(), left:None, right:None})));
    }

    pub fn remove(&mut self, data: &T) {
        self.root=self.root.as_mut().and_then(|root| { root.remove( data ) }).or(None);
    }

    pub fn get_all(&self) -> Vec<T> {
        let mut ret_array=Vec::new();
        if self.root.is_none(){
            return ret_array;
        }

        let mut queue=Vec::new();
        queue.push(self.root.clone().unwrap());
        while !queue.is_empty() {
            let curr=queue.first_mut().unwrap().clone();
            queue.remove(0);
            ret_array.push(curr.data.clone());
            if curr.left.is_some() {
                queue.push(curr.left.clone().unwrap());
            }
            if curr.right.is_some() {
                queue.push(curr.right.clone().unwrap());
            }
        }
        return ret_array;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_contains_one_element_inserted() {
        let bst=BinarySearchTree::create(&0);
        assert!(bst.contains(&0));
    }

    #[test]
    fn it_contains_two_element_inserted() {
        let mut bst=BinarySearchTree::create(&0);
        bst.insert(&1);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
    }

    #[test]
    fn it_contains_three_element_inserted() {
        let mut bst=BinarySearchTree::create(&0);
        bst.insert(&1);
        bst.insert(&2);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(bst.contains(&2));
    }

    #[test]
    fn it_contain_no_not_inserted_element_one_element_inserted() {
        let bst=BinarySearchTree::create(&0);
        assert!(bst.contains(&0));
        assert!(!bst.contains(&13));
    }

    #[test]
    fn it_contain_no_not_inserted_element_two_element_inserted() {
        let mut bst=BinarySearchTree::create(&0);
        bst.insert(&1);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(!bst.contains(&2));
    }

    #[test]
    fn it_contain_no_not_inserted_element_three_element_inserted() {
        let mut bst=BinarySearchTree::create(&0);
        bst.insert(&1);
        bst.insert(&2);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(bst.contains(&2));
        assert!(!bst.contains(&3));
    }

    #[test]
    fn it_contain_no_removed_element_one_element_inserted() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        assert!(bst.contains(&0));
        bst.remove(&0);
        assert!(!bst.contains(&0));
        assert!(!bst.contains(&3));
    }

    #[test]
    fn it_contain_no_removed_element_two_element_inserted() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        bst.insert(&1);
        assert!(bst.contains(&1));
        bst.remove(&1);
        assert!(bst.contains(&0));
        assert!(!bst.contains(&1));
        assert!(!bst.contains(&2));
        assert!(!bst.contains(&3));
    }

    #[test]
    fn it_contain_no_removed_element_three_element_inserted() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        bst.insert(&1);
        bst.insert(&2);
        assert!(bst.contains(&2));
        bst.remove(&2);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(!bst.contains(&2));
        assert!(!bst.contains(&3));
    }

    #[test]
    fn it_contain_no_removed_element_four_element_inserted() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        bst.insert(&1);
        bst.insert(&2);
        bst.insert(&3);
        assert!(bst.contains(&3));
        bst.remove(&3);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(bst.contains(&2));
        assert!(!bst.contains(&3));
        assert!(!bst.contains(&4));
    }

    #[test]
    fn it_contain_no_removed_element_five_element_inserted() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        bst.insert(&1);
        bst.insert(&2);
        bst.insert(&3);
        bst.insert(&4);
        assert!(bst.contains(&4));
        bst.remove(&4);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(bst.contains(&2));
        assert!(bst.contains(&3));
        assert!(!bst.contains(&4));
        assert!(!bst.contains(&5));
    }

    #[test]
    fn it_contain_no_removed_internal_element() {
        let mut bst=BinarySearchTree::new();
        bst.insert(&0);
        bst.insert(&1);
        bst.insert(&2);
        bst.insert(&3);
        bst.insert(&4);
        bst.insert(&5);
        bst.insert(&6);
        bst.insert(&7);
        bst.insert(&8);
        bst.insert(&9);
        assert!(bst.contains(&4));
        bst.remove(&4);
        assert!(bst.contains(&0));
        assert!(bst.contains(&1));
        assert!(bst.contains(&2));
        assert!(bst.contains(&3));
        assert!(!bst.contains(&4));
        assert!(bst.contains(&5));
        assert!(bst.contains(&6));
        assert!(bst.contains(&7));
        assert!(bst.contains(&8));
        assert!(bst.contains(&9));
        assert!(!bst.contains(&10));
    }

    #[test]
    fn it_contain_no_removed_internal_element_2() {
        let mut bst=BinarySearchTree::new();

        for i in 0..10 {
            bst.insert(&i);
        }

        for i in 0..10 {
            assert!(bst.contains(&i));
        }

        for i in (0..10).rev() {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }

        for i in 0..10 {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }

        for i in (0..10).rev() {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }

        for i in 0..10 {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }

        for i in (0..10).rev() {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }


        for i in 0..10 {
            assert!(bst.contains(&i));
            bst.remove(&i);
            assert!(!bst.contains(&i));
            bst.insert(&i);
            assert!(bst.contains(&i));
        }

        for i in 0..10 {
            assert!(bst.contains(&i));
        }
    }
}
