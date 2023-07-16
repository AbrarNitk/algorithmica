pub mod handler;
pub mod utils;
pub struct HttpService;

impl hyper::service::Service<hyper::Request<hyper::Body>> for HttpService {
    type Response = hyper::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = std::pin::Pin<
        Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        Box::pin(async {
            match handler::handle(req).await {
                Ok(r) => Ok(r),
                Err(_e) => {
                    dbg!(_e);
                    unimplemented!()
                }
            }
        })
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    // Creating the tcp listener
    let socket_address: std::net::SocketAddr = ([0, 0, 0, 0], 8000).into();
    let listener = tokio::net::TcpListener::bind(socket_address).await?;
    println!(
        "#### Started at: {}:{} ####",
        socket_address.ip(),
        socket_address.port()
    );
    loop {
        let (tcp_stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = hyper::server::conn::Http::new()
                .serve_connection(tcp_stream, HttpService {})
                .await
            {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}
