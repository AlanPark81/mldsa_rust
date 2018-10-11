use std::cmp::max;
use std::fmt::Debug;

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

    pub fn take_max_value(&mut self, data:&mut T) -> Option<Box<AVLNode<T>>> {
        if self.right.is_none() {
            *data=self.data.clone();
            return self.left.clone();
        } else {
            self.right=self.right.as_mut().unwrap().take_max_value(data);
            return Some(Box::new(self.clone()));
        }
    }

    pub fn insert(&mut self, data:&T) -> Box<AVLNode<T>> {
        if self.data>*data {
            self.left=self.left.as_mut().and_then(| tree | tree.insert(data).balance() ).or(Some( Box::new( AVLNode{data:data.clone(), left:None, right:None} )));
        } else {
            self.right=self.right.as_mut().and_then(| tree | tree.insert(data).balance() ).or(Some( Box::new( AVLNode{data:data.clone(), left:None, right:None} )));
        }
        return self.balance().unwrap();
    }

    pub fn remove(&mut self, data: &T) -> Option< Box < AVLNode<T> > > {
        if self.data==*data {
            if self.left.is_none() && self.right.is_none() {
                return None;
            } else if self.left.is_some() && self.right.is_none() {
                let left = self.left.as_ref().unwrap().clone();

                self.right = left.clone().right;
                self.left = left.clone().left;
                self.data = left.clone().data;

            } else if self.left.is_none() && self.right.is_some() {
                let right = self.right.as_ref().unwrap().clone();

                self.right = right.clone().right;
                self.left = right.clone().left;
                self.data = right.clone().data;
            } else {
                let mut new_data;
                new_data=self.data.clone();
                self.left=self.left.as_mut().unwrap().take_max_value(&mut new_data);
                self.data=new_data;
            }
        } else if self.data<*data && self.right.is_some() {
            self.right=self.right.as_mut().unwrap().remove(data);
        } else if self.data>*data && self.left.is_some() {
            self.left=self.left.as_mut().unwrap().remove(data);
        }

        return self.balance()
    }

    fn level_diff(&self) -> i32 {
        let left_level=self.left.as_ref().and_then(|tree| { Some(tree.level())}).unwrap_or(0) as i32;
        let right_level=self.right.as_ref().and_then(|tree| { Some(tree.level())}).unwrap_or(0) as i32;
        let diff=right_level-left_level;
        if diff.abs() > 1 {
            diff
        } else {
            0
        }
    }

    fn rotate_left(&mut self) -> Option<Box<AVLNode<T>>>{
        let mut right=self.right.take();
        if right.is_none() { return None; }
        self.right=right.as_ref().and_then(|tree| { tree.left.clone() });
        right.as_mut().and_then(|tree| {tree.left=Some(Box::new(self.clone())); tree.left.clone()});
        right
    }

    fn rotate_right(&mut self) -> Option<Box<AVLNode<T>>>{
        let mut left=self.left.take();
        if left.is_none() { return None; }
        self.left=left.as_ref().and_then(|tree| { tree.right.clone() });
        left.as_mut().and_then(|tree| {tree.right=Some(Box::new(self.clone())); tree.right.clone()});
        left
    }

    fn balance(&mut self) -> Option<Box<AVLNode<T>>> {
        let diff=self.level_diff();
        if diff==0 {
            Some( Box::new(self.clone() ) )
        } else if diff > 0 {
            self.rotate_left()
        } else {
            self.rotate_right()
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
}

impl<T> AVLNode<T> where T : Debug {
    pub fn traverse(&self, s:&str) {
        //let mut vec=Vec::new();
        let tree=self.clone();
        println!("{}{:?}", s, tree.data);
        let mut left_tab=String::from(s);
        left_tab.push_str("\t");
        let mut right_tab=String::from(s);
        right_tab.push_str("\t");
        tree.left.as_ref().and_then(|tree| {tree.traverse( left_tab.as_str() ); Some(tree)} );
        tree.right.as_ref().and_then(|tree| {tree.traverse( right_tab.as_str() ); Some(tree)} );
    }
}

pub struct AVLTree<T> where T : Ord + Clone + Debug {
    root:Option<Box<AVLNode<T>>>
}

impl<T> AVLTree<T> where T : Ord + Clone + Debug {
    pub fn new() -> AVLTree<T> {
        AVLTree{
            root: None
        }
    }

    pub fn traverse(&self, s:&str) {
        self.root.as_ref().and_then(|tree| Some(tree.traverse(s)) );
    }

    pub fn remove(&mut self, data: &T) {
        self.root=self.root.as_mut().and_then(|root| { root.remove( data ) }).or(None);
    }

    pub fn insert(&mut self, data:&T) {
        self.root=self.root.as_mut().and_then(|tree| Some(tree.insert(&data.clone()))).or(Some(Box::new(AVLNode::new(data))));
    }

    pub fn level(&self) -> usize {
        self.root.as_ref().and_then(|tree| Some(tree.level())).unwrap_or(0)
    }

    pub fn contains(&self, data: &T) -> bool {
        self.root.as_ref().and_then(|tree| Some( tree.contains( data ) ) ).unwrap_or(false)
    }

    pub fn level_diff(&self) -> i32 {
        self.root.as_ref().and_then(|tree| Some( tree.level_diff() ) ).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;
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
    fn it_has_level_three_for_four_elements_minus_one(){
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
    fn it_has_level_one_for_one_element_minus_one_greatest(){
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
}