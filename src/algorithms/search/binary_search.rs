pub fn binary_search<T>(array:&[T], key:&T) -> Option<usize> where T : Ord {
    if array.len() == 0 {
        return None;
    }
    let index = array.len() / 2;
    if array[ index ] == *key {
        Some( index )
    } else {
        binary_search( &array[0..index], key ).or( binary_search(&array[index+1..], key).and_then(|ret| Some(ret+index+1) ) )
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn search_one() {
        let count= 1;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(binary_search(&array, &0).unwrap(), 0);
    }

    #[test]
    fn search_two() {
        let count= 2;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(binary_search(&array, &0).unwrap(), 0);
    }

    #[test]
    fn search_three() {
        let count= 3;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(binary_search(&array, &0).unwrap(), 0);
    }

    #[test]
    fn search_all_in_five() {
        let count= 5;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }

        for i in 0..count{
            assert_eq!(binary_search(&array, &i).unwrap(), i);
        }
    }

    #[test]
    fn search_not_found() {
        let count= 1;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(binary_search(&array, &1), None);
    }
}