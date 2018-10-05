pub mod avl_tree;
pub mod binary_search_tree;
pub mod heap;
pub mod linked_list;
pub mod merkle_tree;

pub mod merge_sort;
pub mod quick_sort;
pub mod shell_sort;

extern crate blake2_rfc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
