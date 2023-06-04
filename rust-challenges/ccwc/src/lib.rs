fn reader(path: &str) -> Result<std::io::BufReader<std::fs::File>, ()> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        eprintln!("File path does not exists: {:?}", path);
        return Err(());
    }

    let f = std::fs::File::open(path).unwrap();
    Ok(std::io::BufReader::new(f))
}

pub fn word_count(path: &str) -> usize {
    use std::io::BufRead;
    let reader = reader(path).unwrap();
    let mut count: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        count += line.split_whitespace().count();
    }
    count
}

pub fn line_count(path: &str) -> usize {
    use std::io::BufRead;
    let reader = reader(path).unwrap();
    reader.lines().count()
}

pub fn byte_count(path: &str) -> usize {
    use std::io::Read;
    let mut reader = reader(path).unwrap();
    let mut buffer = vec![0; 4096];
    let mut count: usize = 0;
    loop {
        let byte_read_count = reader.read(&mut buffer).unwrap();
        if byte_read_count == 0 {
            break;
        }
        count += byte_read_count;
    }
    count
}

#[cfg(test)]
mod tests {

    #[test]
    fn byte_count() {
        let result = super::byte_count("test.txt");
        assert_eq!(result, 341836);
    }

    #[test]
    fn line_count() {
        let result = super::line_count("test.txt");
        assert_eq!(result, 7137);
    }

    #[test]
    fn word_count() {
        let result = super::word_count("test.txt");
        assert_eq!(result, 58159);
    }
}
