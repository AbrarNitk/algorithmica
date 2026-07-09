use std::fmt::Debug;

unsafe fn get_by_index<T>(list: &[T], index: isize) -> *const T {
    (list.as_ptr() as isize + index as isize * std::mem::size_of::<T>() as isize) as *const T
}

fn merge<T: Debug + Clone, F>(list: &mut [T], start: usize, mid: usize, end: usize, compare: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut left: Vec<T> = Vec::with_capacity(mid - start + 1);
    let mut right: Vec<T> = Vec::with_capacity(end - mid);

    // SAFETY: We're reading elements from a valid slice within bounds.
    // Using clone() instead of ptr::read() to avoid moving ownership out of the Vec.
    unsafe {
        let mut start_idx = start;
        while start_idx <= mid {
            left.push(get_by_index(list, start_idx as isize).read().clone());
            start_idx += 1;
        }
        while start_idx <= end {
            right.push(get_by_index(list, start_idx as isize).read().clone());
            start_idx += 1;
        }
    }

    let mut left_index = 0;
    let mut right_index = 0;
    let mut k = start;

    unsafe {
        while left_index < left.len() && right_index < right.len() {
            if compare(&left[left_index], &right[right_index]) {
                list[k] = get_by_index(&left, left_index as isize).read().clone();
                left_index += 1;
            } else {
                list[k] = get_by_index(&right, right_index as isize).read().clone();
                right_index += 1;
            }
            k += 1;
        }

        while left_index < left.len() {
            list[k] = get_by_index(&left, left_index as isize).read().clone();
            left_index += 1;
            k += 1;
        }

        while right_index < right.len() {
            list[k] = get_by_index(&right, right_index as isize).read().clone();
            right_index += 1;
            k += 1;
        }
    }
}

pub fn merge_sort<T: Debug + Clone, F>(list: &mut [T], start: usize, end: usize, f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if start >= end {
        return;
    }

    let mid = (start + end) / 2;
    merge_sort(list, start, mid, f);
    merge_sort(list, mid + 1, end, f);
    merge(list, start, mid, end, f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut list = vec![10, 20, 10, 4, 5];
        merge_sort(&mut list, 0, list.len() - 1, &|a, b| a.lt(b));
        assert_eq!(list, vec![4, 5, 10, 10, 20]);
    }

    #[test]
    fn test_merge_sort_strings() {
        let mut list = vec!["banana".to_string(), "apple".to_string(), "cherry".to_string()];
        merge_sort(&mut list, 0, list.len() - 1, &|a, b| a.lt(b));
        assert_eq!(list, vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
    }

    #[test]
    fn test_merge_sort_drop() {
        // Regression test for double-free bug
        // This should not panic or cause double-free
        let mut list = vec![String::from("hello"), String::from("world"), String::from("test")];
        merge_sort(&mut list, 0, list.len() - 1, &|a, b| a.lt(b));
        assert_eq!(list, vec!["hello".to_string(), "test".to_string(), "world".to_string()]);
    }
}
