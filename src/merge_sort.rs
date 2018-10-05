use std::collections::LinkedList;
use std::fmt::Debug;
use std::clone::Clone;

fn merge_sort<T>(list:&mut LinkedList<T>) where T : Ord + Debug + Clone{
    let list_len=list.len();
    if list_len == 1 { return;}
    let mut cloned_list=list.clone();
    let mut list_right=cloned_list.split_off((list_len+1)/2);
    let mut list_left=cloned_list.clone();
    merge_sort(&mut list_left);
    merge_sort(&mut list_right);

    list.clear();

    while !list_left.is_empty() && !list_right.is_empty() {
        let item1 = list_left.front().unwrap().clone();
        let item2 = list_right.front().unwrap().clone();
        if item1 < item2 {
            list.push_back(item1);
            list_left.pop_front();
        } else {
            list.push_back(item2);
            list_right.pop_front();
        }
    }
    if !list_left.is_empty() {
        list.append(&mut list_left);
    } else if !list_right.is_empty() {
        list.append(&mut list_right);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_item_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        merge_sort(&mut list);
        let mut before_val=0;
        while !list.is_empty() {
            let curr_val=list.pop_front().unwrap_or(-1);
            if list.is_empty() {
                break;
            }
            assert!(before_val<curr_val);
            before_val=curr_val;
        }
    }

    #[test]
    fn two_item_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        merge_sort(&mut list);
        let mut before_val=0;
        while !list.is_empty() {
            let curr_val=list.pop_front().unwrap_or(-1);
            if list.is_empty() {
                break;
            }
            assert!(before_val<curr_val);
            before_val=curr_val;
        }
    }

    #[test]
    fn three_item_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        merge_sort(&mut list);
        let mut before_val=0;
        while !list.is_empty() {
            let curr_val=list.pop_front().unwrap_or(-1);
            if list.is_empty() {
                break;
            }
            assert!(before_val<curr_val);
            before_val=curr_val;
        }
    }

    #[test]
    fn four_item_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        merge_sort(&mut list);
        let mut before_val=0;
        while !list.is_empty() {
            let curr_val=list.pop_front().unwrap_or(-1);
            if list.is_empty() {
                break;
            }
            assert!(before_val<curr_val);
            before_val=curr_val;
        }
    }

    #[test]
    fn five_item_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        merge_sort(&mut list);
        let mut before_val=0;
        while !list.is_empty() {
            let curr_val=list.pop_front().unwrap_or(-1);
            if list.is_empty() {
                break;
            }
            assert!(before_val<curr_val);
            before_val=curr_val;
        }
    }
}