use std::fmt::Debug;

pub fn bubble_sort<T>(array:&mut [T]) where T : Clone + Copy + Ord + Debug {
    for i in 0..array.len() {
        for j in 0..array.len(){
            if array[i] < array[j] {
                array.swap(i,j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::random;
    use super::*;
    #[test]
    fn bubble_sort_one() {
        let mut array=[0;1];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;1];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        bubble_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn bubble_sort_two() {
        let mut array=[0;2];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;2];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        bubble_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn bubble_sort_three() {
        let mut array=[0;3];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;3];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        bubble_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn bubble_sort_four() {
        let mut array=[0;4];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;4];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        bubble_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn bubble_sort_five() {
        let mut array=[0;5];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;5];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        bubble_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }
}
