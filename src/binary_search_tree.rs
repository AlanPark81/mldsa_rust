use std::boxed::Box;

struct BinarySearchTree<T> {
    left: Option< Box< BinarySearchTree<T> > >,
    right: Option< Box< BinarySearchTree<T> > >,
    data: T
}

impl<T> BinarySearchTree<T> where T : Clone + Ord {
    pub fn new(data:T) ->BinarySearchTree<T> {
        BinarySearchTree{
            left:None,
            right:None,
            data
        }
    }

    pub fn contains(&self, data:T) -> bool {
        if data==self.data{
            return true;
        } else if data < self.data {
            return self.left.as_ref().and_then(|tree| { Some(tree.contains(data))}).unwrap_or(false);
        } else {
            return self.right.as_ref().and_then(|tree| { Some(tree.contains(data))}).unwrap_or(false);
        }
    }

    pub fn insert(&mut self, data:T) {
        if data==self.data{
            return
        }
        if data<self.data {
            let left=self.left.take();
            self.left = left.and_then(
                |mut tree| { tree.insert(data.clone()); Some(tree)})
                .or(Some( Box::new( BinarySearchTree::new( data ) ) ) );
        } else {
            let right=self.right.take();
            self.right = right.and_then(
                |mut tree| { tree.insert(data.clone()); Some(tree)})
                .or(Some( Box::new( BinarySearchTree::new( data ) ) ) );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_contains_one_element_inserted() {
        let bst=BinarySearchTree::new(0);
        assert!(bst.contains(0));
    }

    #[test]
    fn it_contains_two_element_inserted() {
        let mut bst=BinarySearchTree::new(0);
        bst.insert(1);
        assert!(bst.contains(0));
        assert!(bst.contains(1));
    }

    #[test]
    fn it_contains_three_element_inserted() {
        let mut bst=BinarySearchTree::new(0);
        bst.insert(1);
        bst.insert(2);
        assert!(bst.contains(0));
        assert!(bst.contains(1));
        assert!(bst.contains(2));
    }

    #[test]
    fn it_contain_no_not_inserted_element_one_element_inserted() {
        let bst=BinarySearchTree::new(0);
        assert!(bst.contains(0));
        assert!(!bst.contains(13));
    }

    #[test]
    fn it_contain_no_not_inserted_element_two_element_inserted() {
        let mut bst=BinarySearchTree::new(0);
        bst.insert(1);
        assert!(bst.contains(0));
        assert!(bst.contains(1));
        assert!(!bst.contains(2));
    }

    #[test]
    fn it_contain_no_not_inserted_element_three_element_inserted() {
        let mut bst=BinarySearchTree::new(0);
        bst.insert(1);
        bst.insert(2);
        assert!(bst.contains(0));
        assert!(bst.contains(1));
        assert!(bst.contains(2));
        assert!(!bst.contains(3));
    }

    fn it_insert_and_find() {
        let data=vec![1,2,3,4,5,6,7,8,9,10,11,12];
        let mut binary_search_tree=BinarySearchTree::new(0);
        assert!(binary_search_tree.contains(0));
        for input in data.iter() {
            binary_search_tree.insert(input.clone());
        }
        assert!(!binary_search_tree.contains(13));
        for input in data.iter() {
            assert!(binary_search_tree.contains(input.clone()));
        }
    }
}
