pub fn subset_util(
    arr: &[String],
    st: usize,
    end: usize,
    reserve: &mut Vec<String>,
    subsets: &mut Vec<Vec<String>>,
) {
    for index in st..end {
        reserve.push(arr[index].clone());
        subsets.push(reserve.clone());
        subset_util(&arr, index + 1, end, reserve, subsets);
        reserve.pop();
    }
}

pub fn find_all_subset(arr: &[String]) -> Vec<Vec<String>> {
    let mut subsets = vec![];
    let mut reserve = vec![];
    subset_util(arr, 0, arr.len(), &mut reserve, &mut subsets);
    subsets
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn subset_test() {
        let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        assert_eq!(
            find_all_subset(&v),
            vec![
                vec!["A"],
                vec!["A", "B"],
                vec!["A", "B", "C"],
                vec!["A", "C"],
                vec!["B"],
                vec!["B", "C"],
                vec!["C"]
            ]
        );
    }

}
