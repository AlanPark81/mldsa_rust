use std::cmp::max;
pub use data_structures::binary_search_tree::BSTOps;
pub use traits::visitor::{ VisitorAcceptor, Visitor };

#[derive(Clone)]
struct AVLNode<T> {
    data : T,
    left:Option< Box< AVLNode<T> > > ,
    right:Option< Box< AVLNode<T> > >
}

impl<T> AVLNode<T> where T : Ord + Clone {
    pub fn new(data:&T) -> AVLNode<T> {
        AVLNode{
            data:data.clone(),
            left : None,
            right : None
        }
    }

    pub fn take_median_value(&mut self) {
        if self.right.is_some() {
            let mut parent_node = self.right.clone();
            let mut curr_node = parent_node.clone().unwrap().left;
            if curr_node.is_none() {
                self.data=self.right.clone().unwrap().data;
                self.right=None;
                return;
            }
            while curr_node.as_ref().unwrap().left.is_some() {
                parent_node = curr_node.clone();
                curr_node = curr_node.unwrap().left;
            }
            self.data=parent_node.as_ref().unwrap().data.clone();
            parent_node.as_mut().unwrap().left=None;
        }
        else if self.left.is_some() {
            let mut parent_node = self.left.clone();
            let mut curr_node = parent_node.clone().unwrap().right;
            if curr_node.is_none() {
                self.data=self.left.clone().unwrap().data;
                self.left=None;
                return;
            }
            while curr_node.as_ref().unwrap().right.is_some() {
                parent_node = curr_node.clone();
                curr_node = curr_node.unwrap().right;
            }
            self.data=parent_node.as_ref().unwrap().data.clone();
            parent_node.as_mut().unwrap().right=None;
        }
    }

    pub fn insert(&mut self, data:&T) {
        if self.data>*data {
            self.left=self.left.clone().and_then(| mut tree | { tree.insert(data); Some(tree) } ).or(Some( Box::new( AVLNode{data:data.clone(), left:None, right:None} )));
        } else {
            self.right=self.right.clone().and_then(| mut tree | { tree.insert(data); Some(tree) } ).or(Some( Box::new( AVLNode{data:data.clone(), left:None, right:None} )));
        }

        assert!(self.left.as_ref().and_then(|left| Some(left.contains(data)) ).unwrap_or(false) ||
            self.right.as_ref().and_then(|right| Some(right.contains(data)) ).unwrap_or(false) );
    }

    pub fn remove(&mut self, data: &T) -> Option< Box < AVLNode<T> > > {
        if self.data==*data {
            if self.left.is_none() && self.right.is_none() {
                return None;
            } else if self.left.is_some() && self.right.is_none() {
                let left = self.left.as_ref().unwrap().clone();

                self.right = left.right.clone();
                self.left = left.left.clone();
                self.data = left.data.clone();

            } else if self.left.is_none() && self.right.is_some() {
                let right = self.right.as_ref().unwrap().clone();

                self.right = right.right.clone();
                self.left = right.left.clone();
                self.data = right.data.clone();
            } else {
                self.take_median_value();
            }
        } else if self.data<*data && self.right.is_some() {
            self.right=self.right.as_mut().unwrap().remove(data);
        } else if self.data>*data && self.left.is_some() {
            self.left=self.left.as_mut().unwrap().remove(data);
        }

        return Some( Box::new(self.clone() ) );
    }

    fn level_diff(&self) -> i32 {
        let left_level=self.left.as_ref().and_then(|tree| { Some(tree.level())}).unwrap_or(0) as i32;
        let right_level=self.right.as_ref().and_then(|tree| { Some(tree.level())}).unwrap_or(0) as i32;
        let diff=left_level-right_level;
        if diff.abs() > 1 {
            diff
        } else {
            0
        }
    }

    fn rotate_left(&mut self) -> Option<Box<AVLNode<T>>>{
        let mut right=self.right.take();
        if right.is_none() { self.right=right; return None; }
        self.right=right.as_ref().and_then(|tree| { tree.left.clone() });
        right.as_mut().and_then(|tree| {tree.left=Some(Box::new(self.clone())); tree.left.clone()});
        right
    }

    fn rotate_right(&mut self) -> Option<Box<AVLNode<T>>>{
        let mut left=self.left.take();
        if left.is_none() { self.left=left; return None; }
        self.left=left.as_ref().and_then(|tree| { tree.right.clone() });
        left.as_mut().and_then(|tree| {tree.right=Some(Box::new(self.clone())); tree.right.clone()});
        left
    }

    fn balance(&mut self) -> Option<Box<AVLNode<T>>> {
        let diff=self.level_diff();
        if diff < -1 {
            self.right=self.right.as_mut().and_then(|right| {
                let level_diff=right.level_diff();
                let mut right_new=right.clone();
                if level_diff < -1 {
                    right_new=right.rotate_left().unwrap_or(right_new);
                } else if level_diff >= 0 {
                    right_new=right.rotate_right().unwrap_or(right_new);
                }
                Some(right_new.clone())
            }).or(None);
            return self.rotate_left()
        } else if diff > 1{
            self.left=self.left.as_mut().and_then(|left| {
                let level_diff=left.level_diff();
                let mut left_new=left.clone();
                if level_diff > 1 {
                    left_new=left.rotate_right().unwrap_or(left_new);
                } else if level_diff <= 0 {
                    left_new=left.rotate_left().unwrap_or(left_new);
                }
                Some(left_new.clone())
            }).or(None);
            return self.rotate_right()
        } else {
            Some( Box::new(self.clone() ) )
        }
    }

    pub fn level(&self) -> usize {
        let left_level = self.left.as_ref().and_then(|tree| Some(tree.level()) ).unwrap_or(0);
        let right_level= self.right.as_ref().and_then(|tree| Some(tree.level()) ).unwrap_or(0);
        max(left_level, right_level)+1
    }

    pub fn contains(&self, data: &T) -> bool {
        self.data == *data
        || self.left.as_ref().and_then(|tree| Some(tree.contains(data)) ).unwrap_or(false)
        || self.right.as_ref().and_then(|tree| Some(tree.contains(data)) ).unwrap_or(false)
    }

    pub fn get_all_sorted(&self) -> Vec<T> {
        let mut queue=Vec::new();
        let mut left_queue=self.left.as_ref().and_then(|node| Some(node.get_all_sorted()) ).unwrap_or(Vec::new());
        let mut right_queue=self.right.as_ref().and_then(|node| Some(node.get_all_sorted()) ).unwrap_or(Vec::new());
        queue.append(&mut left_queue);
        queue.push(self.data.clone());
        queue.append(&mut right_queue);
        return queue;
    }
}

pub struct AVLTree<T> where T : Ord + Clone {
    root:Option<Box<AVLNode<T>>>
}

impl<T> AVLTree<T> where T : Ord + Clone {
    pub fn new() -> AVLTree<T> {
        AVLTree {
            root: None
        }
    }

    pub fn level(&self) -> usize {
        self.root.as_ref().and_then(|tree| Some(tree.level())).unwrap_or(0)
    }

    pub fn level_diff(&self) -> i32 {
        self.root.as_ref().and_then(|tree| Some( tree.level_diff() ) ).unwrap_or(0)
    }
}

impl<T> BSTOps<T> for AVLTree<T> where T : Ord + Clone {
    fn contains(&self, data: &T) -> bool {
        self.root.as_ref().and_then(|tree| Some( tree.contains( data ) ) ).unwrap_or(false)
    }

    fn insert(&mut self, data:&T) {
        self.root=self.root.as_mut()
            .and_then(|tree| {
                tree.insert(&data.clone());
                return tree.balance() } )
            .or(Some(Box::new(AVLNode::new(data))));
    }

    fn remove(&mut self, data: &T) {
        self.root=self.root.as_mut().and_then(|root| {
            root.remove( data ).as_mut().and_then(|tree| tree.balance() )
        }).or(None);
    }

    fn get_breadth_first(&self) -> Vec<T> {
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

    fn get_all_sorted(&self) -> Vec<T> {
        self.root.as_ref().and_then(|root| Some(root.get_all_sorted())).unwrap_or(Vec::new())
    }
}

impl<'a, T> VisitorAcceptor<T, &'a str> for AVLNode<T> where T : Ord + Clone {
    fn accept<V>(&mut self, visitor:&mut V) -> Result<(), &'a str> where V : Visitor<T, &'a str> {
        self.data=visitor.visit(&self.data)?;
        Ok(())
    }
}

impl<'a, T> VisitorAcceptor<T, &'a str> for AVLTree<T> where T : Ord + Clone {
    fn accept<V>(&mut self, visitor:&mut V) -> Result<(), &'a str> where V : Visitor<T, &'a str> {
        let mut queue = Vec::new();
        queue.push(self.root.clone());
        while !queue.is_empty() {
            queue.swap_remove(0).as_mut().and_then(|node| {
                queue.push(node.left.clone());
                queue.push(node.right.clone());
                node.accept(visitor);
                Some(())
            });
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_has_level_one_for_one_element(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        assert_eq!(avl_tree.level(),1);
    }

    #[test]
    fn it_has_level_two_for_two_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&3);
        assert_eq!(avl_tree.level(),2);
    }

    #[test]
    fn it_has_level_two_for_three_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&3);
        assert_eq!(avl_tree.level(),2);
    }

    #[test]
    fn it_has_level_three_for_four_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&2);
        avl_tree.insert(&3);

        assert_eq!(avl_tree.level(),3);
    }

    #[test]
    fn it_has_level_one_for_one_element_minus_one(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),0);
        assert!(!avl_tree.contains(&0));
    }

    #[test]
    fn it_has_level_two_for_two_elements_minus_one(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&3);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),1);
        assert!(!avl_tree.contains(&0));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_two_for_three_elements_minus_one(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&3);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),2);
        assert!(!avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_two_for_four_elements_minus_one(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&2);
        avl_tree.insert(&3);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),2);
        assert!(!avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(avl_tree.contains(&2));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_zero_for_one_element_minus_one_greatest(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),0);
        assert!(!avl_tree.contains(&0));
    }

    #[test]
    fn it_has_level_two_for_two_elements_minus_one_greatest(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&3);
        avl_tree.remove(&0);
        assert_eq!(avl_tree.level(),1);
        assert!(!avl_tree.contains(&0));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_two_for_three_elements_minus_one_greatest(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&3);
        avl_tree.remove(&3);
        assert_eq!(avl_tree.level(),2);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(!avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_three_for_four_elements_minus_one_greatest(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&2);
        avl_tree.insert(&3);
        avl_tree.remove(&3);
        assert_eq!(avl_tree.level(),2);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(avl_tree.contains(&2));
        assert!(!avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_two_for_three_elements_minus_one_media(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&3);
        avl_tree.remove(&1);
        assert_eq!(avl_tree.level(),2);
        assert!(avl_tree.contains(&0));
        assert!(!avl_tree.contains(&1));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_has_level_three_for_four_elements_minus_one_median(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&2);
        avl_tree.insert(&3);
        avl_tree.remove(&2);
        assert_eq!(avl_tree.level(),2);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(!avl_tree.contains(&2));
        assert!(avl_tree.contains(&3));

        avl_tree.remove(&1);
        assert_eq!(avl_tree.level(),2);
        assert!(avl_tree.contains(&0));
        assert!(!avl_tree.contains(&1));
        assert!(!avl_tree.contains(&2));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_contains_one_element() {
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&1);
        assert!(avl_tree.contains(&1));
    }

    #[test]
    fn it_contains_two_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&3);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_contains_three_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&3);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_contains_four_elements(){
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        avl_tree.insert(&1);
        avl_tree.insert(&2);
        avl_tree.insert(&3);
        assert!(avl_tree.contains(&0));
        assert!(avl_tree.contains(&1));
        assert!(avl_tree.contains(&2));
        assert!(avl_tree.contains(&3));
    }

    #[test]
    fn it_contains_no_not_included_element() {
        let mut avl_tree=AVLTree::new();
        avl_tree.insert(&0);
        assert!(!avl_tree.contains(&13));
    }

    #[test]
    fn it_get_all_and_get_all_sorted() {
        let mut bst=AVLTree::new();
        bst.insert(&5);
        bst.insert(&3);
        bst.insert(&7);
        bst.insert(&2);
        bst.insert(&4);
        bst.insert(&6);
        bst.insert(&8);
        assert_eq!(bst.get_breadth_first(), vec![5,3,7,2,4,6,8]);
        assert_eq!(bst.get_all_sorted(), vec![2,3,4,5,6,7,8]);
    }

    #[test]
    fn balance_test_insert() {
        let mut avl_tree=AVLTree::new();
        for i in 0..100 {
            avl_tree.insert(&i);
            assert!( avl_tree.root.as_ref().and_then(|root| {
                let diff=root.level_diff();
                Some(diff<2 && diff>-2)
            } ).unwrap_or(true) );
        }

        for i in 0..100 {
            assert!(avl_tree.contains(&i));
        }
    }

    #[test]
    fn balance_test_remove() {
        let mut avl_tree=AVLTree::new();
        for i in 0..100 {
            avl_tree.insert(&i);
            assert!( avl_tree.root.as_ref().and_then(|root| {
                let diff=root.level_diff();
                Some(diff<2 && diff>-2)
            } ).unwrap_or(true) );
        }

        for i in 0..100 {
            assert!(avl_tree.contains(&i));
        }

        for i in 0..100 {
            avl_tree.remove(&i);
            assert!( avl_tree.root.as_ref().and_then(|root| {
                let diff=root.level_diff();
                Some(diff<2 && diff>-2)
            } ).unwrap_or(true) );
        }

        for i in 0..100 {
            assert!( !avl_tree.contains(&i) );
        }
    }

    #[test]
    fn visitor_test() {
        struct FootprintsVisitor {
            pub footprints : Vec<u32>,
        }

        impl FootprintsVisitor {
            fn new() -> FootprintsVisitor { FootprintsVisitor {
                footprints : Vec::new()
            }}
        }

        impl<'a> Visitor<u32, &'a str> for FootprintsVisitor {
            fn visit(&mut self, data:&u32) -> Result<u32, &'a str> {
                self.footprints.push(data.clone());
                Ok(data.clone())
            }
        }

        let mut tree=AVLTree::new();

        for i in 0..10 {
            tree.insert(&i);
        }
        let mut visitor=FootprintsVisitor::new();
        tree.accept(&mut visitor);

        let expected_seq= vec![5, 3, 6, 4, 7, 8, 9, 1, 0, 2];

        assert_eq!(visitor.footprints.len(), 10);
        for i in 0..visitor.footprints.len() {
            assert_eq!(visitor.footprints[i], expected_seq[i]);
        }
    }
}
