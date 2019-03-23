pub fn multiply<Matrix: AsRef<[Row]>, Row: AsRef<[f32]>>(
    mat1: &Matrix,
    mat2: &Matrix,
) -> Vec<Vec<f32>> {
    let result = vec![vec![]];
    for row in mat1.as_ref() {
        for cell in row.as_ref() {
            print!("{} ", cell);
        }
        println!();
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul() {
        let t1 = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        multiply(&t1, &t1);
        println!("{:?}", t1);
    }
}
