pub trait CreateSet {
    fn create_set() -> Self;
}

pub trait Contains<T> where T : Eq {
    fn contains(&self, data: &T) -> bool;
}

pub trait Insert<T> where T : Eq + Clone {
    fn insert(&mut self, data:&T);
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

pub trait GetAllElements<T> where T : Eq + Clone {
    fn get_all_elements(&self) -> Vec<T>;
}

pub trait Set<T> : CreateSet + Contains<T> + Insert<T> + IsEmpty + GetAllElements<T> + Sized
    where T : Eq + Clone {
    fn intersect<V>(set_a:&Self, set_b:&V) -> Self where V : Set<T> {
        let elements_in_a = set_a.get_all_elements();
        let mut set_to_return=Self::create_set();
        for data in elements_in_a {
            if set_b.contains(&data) {
                set_to_return.insert(&data);
            }
        }
        return set_to_return;
    }
    fn subset(&self, f: &Fn(&T)->bool ) -> Self {
        let all_elements = self.get_all_elements();
        let mut subset = Self::create_set();
        for element in all_elements {
            if f(&element) {
                subset.insert(&element);
            }
        }
        subset
    }
}

#[cfg(test)]
mod tests {
    use data_structures::avl_tree::AVLTree;
    use super::*;
    #[test]
    fn avl_tree_as_set() {
        let mut set= AVLTree::create_set();
        for i in 0..10 {
            set.insert(&i);
        }

        for i in 0..10 {
            assert!(set.contains(&i));
        }
    }

    #[test]
    fn intersect() {
        let mut set_a= AVLTree::create_set();
        for i in 0..10 {
            set_a.insert(&i);
        }
        let mut set_b =AVLTree::create_set();
        for i in 5..20 {
            set_b.insert(&i);
        }

        let intersect = AVLTree::intersect(&set_a, &set_b);

        let elements_intersect = intersect.get_all_elements();
        let elements_a = set_a.get_all_elements();
        let elements_b = set_b.get_all_elements();

        for data in elements_intersect {
            assert!(set_a.contains(&data) && set_b.contains(&data));
        }

        for data in elements_a {
            if set_b.contains(&data) {
                assert!(intersect.contains(&data));
            }
        }

        for data in elements_b {
            if set_a.contains(&data) {
                assert!(intersect.contains(&data));
            }
        }
    }
}