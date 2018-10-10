use std::fmt::Debug;

macro_rules! compare_swap {
    ($array:expr, $index1:expr, $index2:expr) => {{
        if $array[$index1] < $array[$index2] {
            $array.swap($index1, $index2);
        }
    }}
}

pub fn quick_sort<T>(array:&mut [T]) where T : Clone + Copy + Ord + Debug{
    if array.len()<=1 {
        return;
    }
    let hi_index=array.len()-1;
    let low_index = 0 ;
    let mid_index=(hi_index+low_index)/2;
    if array.len()>2{
        compare_swap!(array, mid_index, low_index);
        compare_swap!(array, hi_index, low_index);
        compare_swap!(array, mid_index, hi_index);
    }

    let pivot = array[hi_index];
    let pivot_index;
    loop {
        let mut bigger_first=0;
        while bigger_first<array.len() && array[bigger_first] < pivot {
            bigger_first+=1;
        }

        let mut less_first=hi_index;
        while array[less_first] > pivot {
            less_first-=1;
        }
        if bigger_first >= less_first {
            pivot_index=less_first;
            break;
        }
        array.swap(less_first, bigger_first);
    }
    quick_sort(&mut array[0..pivot_index]);
    quick_sort(&mut array[pivot_index+1..]);
}


#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::random;
    use super::*;
    #[test]
    fn quick_sort_one() {
        let mut array=[0;1];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;1];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        quick_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn quick_sort_two() {
        let mut array=[0;2];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;2];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        quick_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn quick_sort_three() {
        let mut array=[0;3];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;3];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        quick_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn quick_sort_four() {
        let mut array=[0;4];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;4];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        quick_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn quick_sort_five() {
        let mut array=[0;5];
        for i in 0..array.len() {
            array[i]=random();
        }
        let mut array_sorted=[0i32;5];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        quick_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }
}
