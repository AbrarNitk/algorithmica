fn binary_search_util<T>(list: &[T], element: &T, start: isize, end: isize) -> bool
where
    T: PartialOrd,
{
    if end < start {
        return false;
    }
    let mid = start + (end - start) / 2;
    if &list[mid as usize] == element {
        return true;
    }
    if list[mid as usize].gt(element) {
        return binary_search_util(list, element, start, mid - 1);
    }
    binary_search_util(list, element, mid + 1, end)
}

pub fn search<T>(list: &[T], element: &T) -> bool
where
    T: PartialOrd,
{
    !list.is_empty() && binary_search_util(list, element, 0, (list.len() - 1) as isize)
}

fn search_rotated_util<T>(list: &[T], element: &T, start: usize, end: usize) -> bool
where
    T: PartialOrd,
{
    if start > end {
        return false;
    }
    let mid = (start + end) / 2;
    if &list[mid] == element {
        return true;
    }

    if list[mid] < list[end] {
        if &list[mid] < element && element <= &list[end] {
            search_rotated_util(list, element, mid + 1, end)
        } else {
            search_rotated_util(list, element, start, mid - 1)
        }
    } else {
        if &list[start] <= element && element <= &list[mid] {
            search_rotated_util(list, element, start, mid - 1)
        } else {
            search_rotated_util(list, element, mid + 1, end)
        }
    }
}

// [6, 7, 1, 2, 3, 4, 5]
pub fn search_rotated<T>(list: &[T], element: &T) -> bool
where
    T: PartialOrd,
{
    if list.is_empty() {
        return false;
    }

    return search_rotated_util(list, element, 0, list.len() - 1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search_rotated() {
        let v = vec![
            176, 188, 199, 200, 210, 222, 1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155,
            162,
        ];
        assert!(super::search_rotated(v.as_slice(), &1));
        assert!(super::search_rotated(v.as_slice(), &222));
        assert!(super::search_rotated(v.as_slice(), &88));
        assert!(super::search_rotated(v.as_slice(), &200));
        assert!(super::search_rotated(v.as_slice(), &162));
        assert!(super::search_rotated(v.as_slice(), &176));
        assert!(!super::search_rotated(v.as_slice(), &175));
        assert!(!super::search_rotated(
            vec![6, 7, 1, 2, 3, 4, 5, 6].as_slice(),
            &176
        ));
        assert!(super::search_rotated(
            vec![6, 7, 1, 2, 3, 4, 5].as_slice(),
            &1
        ));
    }
}
