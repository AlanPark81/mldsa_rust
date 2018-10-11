pub fn fixed_point(array:&[u32]) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let mut lo = 0;
    let mut hi = array.len()-1;
    while hi>=lo {
        let mid = (hi-lo)/2 + lo;
        if mid == array[mid] as usize {
            return Some(mid);
        } else if mid > array[mid] as usize {
            lo = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            hi = mid - 1;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::fixed_point;

    #[test]
    fn search_one(){
        let count= 1;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(fixed_point(&array).unwrap(), 0);
    }

    #[test]
    fn search_two(){
        let count= 2;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(fixed_point(&array).unwrap(), 0);
    }

    #[test]
    fn search_three(){
        let count= 3;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i);
        }
        assert_eq!(fixed_point(&array).unwrap(), 1);
    }

    #[test]
    fn search_not_contiguous_three(){
        let count= 3;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i*2);
        }
        assert_eq!(fixed_point(&array).unwrap(), 0);
    }

    #[test]
    fn search_not_contiguous_five(){
        let count= 5;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i*2);
        }
        assert_eq!(fixed_point(&array).unwrap(), 0);
    }

    #[test]
    fn search_not_found(){
        let count= 5;
        let mut array=Vec::new();
        for i in 0..count {
            array.push(i+1);
        }
        assert_eq!(fixed_point(&array), None);
    }
}