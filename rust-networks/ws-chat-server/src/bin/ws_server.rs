type Tx = tokio::sync::broadcast::Sender<tungstenite::Message>;
type Rx = tokio::sync::broadcast::Receiver<tungstenite::Message>;

struct Client {
    id: usize,
    stream: std::sync::Arc<std::net::TcpStream>,
    receiver: std::sync::Arc<std::net::TcpStream>,
}
fn main() {}
