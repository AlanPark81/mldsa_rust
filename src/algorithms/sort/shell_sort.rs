pub fn shell_sort<T>(array: &mut [T]) where T : Ord + Clone {
    let mut gaps=vec![701, 301, 132, 57, 23, 10, 4, 1];
    let array_len=array.len();
    for i in 0..gaps.len() {
        if gaps[i] < array_len {
            gaps=gaps[i..].to_vec();
            break;
        }
    }

    for gap in gaps {
        for i in gap..array_len {
            let temp=array[i].clone();
            let mut j=i;
            while j>=gap && array[j-gap]>temp {
                array[j]=array[j-gap].clone();
                j-=gap;
            }
            array[j]=temp;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::shell_sort;
    extern crate rand;
    use self::rand::{ thread_rng, RngCore };
    #[test]
    fn shell_sort_one() {
        const COUNT:usize=1;
        let mut array=[0u32;COUNT];
        let mut vec=Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            array[i]=thread_rng().next_u32();
            vec.push(array[i].clone());
        }
        vec.sort();

        shell_sort(&mut array);
        for i in 0..COUNT {
            assert_eq!(array[i], vec[i]);
        }
    }

    #[test]
    fn shell_sort_two() {
        const COUNT:usize=2;
        let mut array=[0u32;COUNT];
        let mut vec=Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            array[i]=thread_rng().next_u32();
            vec.push(array[i].clone());
        }
        vec.sort();

        shell_sort(&mut array);
        for i in 0..COUNT {
            assert_eq!(array[i], vec[i]);
        }
    }

    #[test]
    fn shell_sort_three() {
        const COUNT:usize=3;
        let mut array=[0u32;COUNT];
        let mut vec=Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            array[i]=thread_rng().next_u32();
            vec.push(array[i].clone());
        }
        vec.sort();

        shell_sort(&mut array);
        for i in 0..COUNT {
            assert_eq!(array[i], vec[i]);
        }
    }

    #[test]
    fn shell_sort_four() {
        const COUNT:usize=4;
        let mut array=[0u32;COUNT];
        let mut vec=Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            array[i]=thread_rng().next_u32();
            vec.push(array[i].clone());
        }
        vec.sort();

        shell_sort(&mut array);
        for i in 0..COUNT {
            assert_eq!(array[i], vec[i]);
        }
    }

    #[test]
    fn shell_sort_five() {
        const COUNT:usize=5;
        let mut array=[0u32;COUNT];
        let mut vec=Vec::with_capacity(COUNT);
        for i in 0..COUNT {
            array[i]=thread_rng().next_u32();
            vec.push(array[i].clone());
        }
        vec.sort();

        shell_sort(&mut array);
        for i in 0..COUNT {
            assert_eq!(array[i], vec[i]);
        }
    }
}
