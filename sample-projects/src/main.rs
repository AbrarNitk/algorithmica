#[tokio::main]
pub async fn main() {
    md_renderer::main().await.expect("Error in running main");
}
