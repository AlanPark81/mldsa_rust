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
