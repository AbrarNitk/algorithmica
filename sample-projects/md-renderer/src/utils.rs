pub async fn read_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    let path = current_dir.join("sample-projects").join(path);
    println!("{:?}", path);
    tokio::fs::read_to_string(path).await
}
