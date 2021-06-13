fn sort<T: Ord>(arr: &mut [T]) {
    sort_by(arr, |a, b| a.lt(b))
}

fn sort_by<T: Ord, F: Fn(&T, &T) -> bool>(arr: &mut [T], p: F) {
    for x in (0..=(arr.len() - 1) / 2).rev() {
        sink(arr, x, (arr.len() - 1), &p);
    }

    let k = 0;
    let mut l = arr.len() - 1;
    while l > 0 {
        arr.swap(k, l);
        l = l - 1;
        sink(arr, 0, l, &p);
    }
}

pub fn swim<T: Ord>(arr: &mut [T], k: usize) {
    let mut k = k;
    while k > 0 {
        let parent = (k - 1) / 2;
        if arr[parent].lt(&arr[k]) {
            arr.swap(parent, k);
            k = parent;
        } else {
            break;
        }
    }
}

pub fn sink<T: Ord, F: Fn(&T, &T) -> bool>(arr: &mut [T], k: usize, n: usize, p: &F) {
    let mut k = k;
    while 2 * k + 1 <= n {
        let mut j = 2 * k + 1;
        if j + 1 <= n && p(&arr[j], &arr[j + 1]) {
            j += 1;
        }
        if !p(&arr[j], &arr[k]) {
            arr.swap(k, j);
            k = j;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swim_test_1() {
        let mut v = vec![5, 4, 3, 2, 1, 6, 2];
        swim(&mut v, 5);
        assert_eq!(v, vec![6, 4, 5, 2, 1, 3, 2]);
    }

    #[test]
    fn sink_test_1() {
        let mut v = vec![1, 7, 4, 5, 4, 3, 2, 2];
        sink(&mut v, 0, 7, &|a, b| a.lt(b));
        assert_eq!(v, vec![7, 5, 4, 2, 4, 3, 2, 1]);
    }

    #[test]
    fn sink_test_2() {
        let mut v = vec![1, 3, 5, 2, 1, 4, 3];
        sink(&mut v, 0, 6, &|a, b| a.lt(b));
        assert_eq!(v, vec![5, 3, 4, 2, 1, 1, 3]);
    }

    #[test]
    fn sort_test_1() {
        let mut v = vec![1, 3, 5, 2, 1, 4, 3];
        sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5])
    }

    #[test]
    fn sort_test_2() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        v.reverse();
        sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn sort_test_3() {
        let mut v = vec![1, 2, 3, 2, 3, 6, 6];
        v.reverse();
        sort(&mut v);
        assert_eq!(v, vec![1, 2, 2, 3, 3, 6, 6])
    }
}
