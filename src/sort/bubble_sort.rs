use std::cmp::{Ordering, PartialEq, PartialOrd};

pub fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd + PartialEq,
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut flag: bool = true;
        for j in 0..n - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                flag = false;
            }
        }
        if flag {
            break;
        }
    }
}

pub fn bubble_sort_by<T, F>(list: &mut [T], f: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut flag: bool = true;
        for j in 0..n - i - 1 {
            if let Ordering::Greater = f(&list[j], &list[j + 1]) {
                list.swap(j, j + 1);
                flag = false;
            }
        }
        if flag {
            break;
        }
    }
}
