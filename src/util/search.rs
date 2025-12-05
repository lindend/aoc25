/// Find the index of the first element in a sorted list that is larger than or
/// equal to the find argument 
/// 
/// # Arguments 
/// 
/// * `list`: A sorted list
/// * `find`: The element to find
/// 
/// returns: usize 
/// 
/// # Examples 
/// 
/// ```
/// assert_eq!(binary_search_leftmost([1, 2, 3, 4], 2), 1);
/// ```
pub fn binary_search_leftmost<T: PartialOrd>(list: &[T], find: T) -> usize {
    let mut min = 0;
    let mut max = list.len() - 1;

    loop {
        if min == max {
            return min;
        }

        let pivot = (min + max) / 2;

        if list[pivot] == find {
            return pivot;
        } else if list[pivot] > find {
            max = pivot;
        } else {
            min = pivot + 1
        }
    }
}

/// The same as leftmost, but will find the first element smaller than or equal to find
pub fn binary_search_rightmost<T: PartialOrd>(list: &[T], find: T) -> usize {
    let mut min = 0;
    let mut max = list.len();

    loop {
        if min == max {
            if max == 0 {
                return 0;
            }
            return max - 1;
        }

        let pivot = (min + max) / 2;

        if list[pivot] == find {
            return pivot;
        } else if list[pivot] > find {
            max = pivot;
        } else {
            min = pivot + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let l = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(binary_search_leftmost(&l, 3), 2);
        assert_eq!(binary_search_rightmost(&l, 3), 2);
    }

    #[test]
    fn test_find_first() {
        let l = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(binary_search_leftmost(&l, 1), 0);
        assert_eq!(binary_search_rightmost(&l, 1), 0);
    }

    #[test]
    fn test_find_last() {
        let l = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(binary_search_leftmost(&l, 7), 6);
        assert_eq!(binary_search_rightmost(&l, 7), 6);
    }

    #[test]
    fn test_find_before_beginning() {
        let l = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(binary_search_leftmost(&l, 0), 0);
        assert_eq!(binary_search_rightmost(&l, 0), 0);
    }

    #[test]
    fn test_find_past_end() {
        let l = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(binary_search_leftmost(&l, 8), 6);
        assert_eq!(binary_search_rightmost(&l, 8), 6);
    }

    #[test]
    fn test_find_non_existent() {
        let l = [0, 2, 4, 6, 8, 10];
        assert_eq!(binary_search_leftmost(&l, 1), 1);
        assert_eq!(binary_search_leftmost(&l, 3), 2);
        assert_eq!(binary_search_leftmost(&l, 5), 3);
        assert_eq!(binary_search_leftmost(&l, 7), 4);
        assert_eq!(binary_search_leftmost(&l, 9), 5);
        assert_eq!(binary_search_leftmost(&l, 11), 5);
        assert_eq!(binary_search_leftmost(&l, 12), 5);
        assert_eq!(binary_search_leftmost(&l, 13), 5);

        assert_eq!(binary_search_rightmost(&l, 1), 0);
        assert_eq!(binary_search_rightmost(&l, 3), 1);
        assert_eq!(binary_search_rightmost(&l, 5), 2);
        assert_eq!(binary_search_rightmost(&l, 7), 3);
        assert_eq!(binary_search_rightmost(&l, 9), 4);
        assert_eq!(binary_search_rightmost(&l, 11), 5);
        assert_eq!(binary_search_rightmost(&l, 12), 5);
        assert_eq!(binary_search_rightmost(&l, 13), 5);
    }
}