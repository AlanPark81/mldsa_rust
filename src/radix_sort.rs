use std::fmt::Debug;
use std::vec::Vec;
use std::u64::MAX;
pub fn radix_sort(array:&mut Vec<u64>){
    let mut radix_queue = Vec::new();
    for _ in 0..10 {
        radix_queue.push(Vec::new());
    }
    let mut index=1;
    let limit=u64::max_value()/10;
    while index <= limit {
        for data in array.clone() {
            radix_queue[ (data/index%10) as usize].push(data);
        }
        if radix_queue.len() == array.len() {
            break;
        }
        let mut vector=Vec::new();
        for mut vec in &mut radix_queue {
            vector.append(&mut vec);
        }
        *array = vector;
        println!("{} {:?}", index,  array);
        index*=10;
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::random;
    use super::*;
    #[test]
    fn radix_sort_one() {
        let mut array=vec![0u64;1];
        for i in 0..array.len() {
            array[i]=random::<u32>().into();
        }
        let mut array_sorted=vec![0u64;1];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        radix_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn radix_sort_two() {
        let mut array=vec![0u64;2];
        for i in 0..array.len() {
            array[i]=random::<u32>().into();
        }
        let mut array_sorted=vec![0u64;2];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        radix_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn radix_sort_three() {
        let mut array=vec![0u64;3];
        for i in 0..array.len() {
            array[i]=random::<u32>().into();
        }
        let mut array_sorted=vec![0u64;3];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        radix_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn radix_sort_four() {
        let mut array=vec![0u64;4];
        for i in 0..array.len() {
            array[i]=random::<u32>().into();
        }
        let mut array_sorted=vec![0u64;4];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        radix_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }

    #[test]
    fn radix_sort_five() {
        let mut array=vec![0u64;5];
        for i in 0..array.len() {
            array[i]=random::<u32>().into();
        }
        let mut array_sorted=vec![0u64;5];
        array_sorted.clone_from_slice(&array);
        array_sorted.sort();
        radix_sort(&mut array);

        for i in 0..array.len() {
            if i>=1 {
                assert!(array[i]>array[i-1]);
            }
            assert_eq!(array[i], array_sorted[i]);
        }
    }
}
