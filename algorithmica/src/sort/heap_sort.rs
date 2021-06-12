fn sort_by(arr: &mut [i32]) {}

fn sort(arr: &mut [i32]) {
    for x in (0..=(arr.len() - 1) / 2).rev() {
        sink(arr, x, (arr.len() - 1));
    }
    let k = 0;
    let mut l = arr.len() - 1;
    while l > 0 {
        let temp = arr[k];
        arr[k] = arr[l];
        arr[l] = temp;
        l = l - 1;
        sink(arr, 0, l);
    }
}

pub fn swim(arr: &mut [i32], k: usize) {
    let mut k = k;
    while k > 0 {
        let parent = (k - 1) / 2;
        if arr[parent] < arr[k] {
            let temp = arr[parent];
            arr[parent] = arr[k];
            arr[k] = temp;
            k = parent;
        } else {
            break;
        }
    }
}

pub fn sink(arr: &mut [i32], k: usize, n: usize) {
    let mut k = k;
    while 2 * k + 1 <= n {
        let mut j = 2 * k + 1;
        if j + 1 <= n && arr[j] < arr[j + 1] {
            j += 1;
        }
        if arr[j] > arr[k] {
            let temp = arr[j];
            arr[j] = arr[k];
            arr[k] = temp;
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
        sink(&mut v, 0, 7);
        assert_eq!(v, vec![7, 5, 4, 2, 4, 3, 2, 1]);
    }

    #[test]
    fn sink_test_2() {
        let mut v = vec![1, 3, 5, 2, 1, 4, 3];
        sink(&mut v, 0, 6);
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
}
