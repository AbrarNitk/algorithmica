use std::cmp::Ord;

fn quick_sort<T>(list: &mut [T], start: usize, end: usize)
where
    T: Ord,
{
    if start >= end {
        return;
    }

    let mut i = start;
    let mut j = start;

    while j < end {
        if list[j].lt(&list[end]) {
            list.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    list.swap(i, end);

    if i > 0 {
        quick_sort(list, start, i - 1);
    }
    quick_sort(list, i + 1, end);
}

pub fn sort<T>(list: &mut [T])
where
    T: Ord,
{
    if list.is_empty() || list.len() == 1 {
        return;
    }
    quick_sort(list, 0, list.len() - 1);
}
