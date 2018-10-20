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

pub trait Set<T> : CreateSet + Contains<T> + Insert<T> + IsEmpty + GetAllElements<T>
    where T : Eq + Clone {
    fn intersect<U, V, W>(set_a:&U, set_b:&V) -> W where U : Set<T>, V : Set<T>, W : Set<T>;
    fn subset<S>(&self, f: &Fn(T)->bool ) -> S where S : Set<T>;
}