use std::boxed::Box;

struct Node<T> {
    left: Option< Box< BinarySearchTree<T> > >,
    right: Option< Box< BinarySearchTree<T> > >,
    data: T
}

pub struct BinarySearchTree<T> {
    root:Option< Box< Node<T> > >
}

impl<T> BinarySearchTree<T> where T : Clone + Ord {
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

        if *data==self.root.as_ref().unwrap().data{
            return
        }
        if *data<self.root.as_ref().unwrap().data {
            let left=self.root.as_mut().unwrap().left.take();
            self.root.as_mut().unwrap().left = left.and_then(
                |mut tree| { tree.insert( data ); Some(tree)})
                .or(Some( Box::new( BinarySearchTree::create( data ) ) ) );
        } else {
            let right=self.root.as_mut().unwrap().right.take();
            self.root.as_mut().unwrap().right = right.and_then(
                |mut tree| { tree.insert( data ); Some(tree)})
                .or(Some( Box::new( BinarySearchTree::create( data ) ) ) );
        }
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
}
