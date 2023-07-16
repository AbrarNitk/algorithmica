pub async fn test() -> Result<String, ()> {
    let content = crate::utils::read_file(std::path::Path::new("test.md"))
        .await
        .expect("can not read the file path");
    let adapter = comrak::plugins::syntect::SyntectAdapter::new("base16-ocean.dark");
    let mut plugins = comrak::ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    let options = comrak::ComrakOptions::default();
    let output = comrak::markdown_to_html_with_plugins(content.as_str(), &options, &plugins);
    Ok(output)
}
pub async fn handle(req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, ()> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => {
            let mut response = hyper::Response::new(hyper::Body::empty());
            // let resp = http_service::controller::get_user_profile()?;
            *response.body_mut() = hyper::Body::from(test().await?);
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("text/html").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        (&hyper::Method::POST, "/") => {
            let mut response = hyper::Response::new(hyper::Body::empty());
            *response.body_mut() = hyper::Body::from("POST Response");
            *response.status_mut() = hyper::StatusCode::OK;
            response.headers_mut().append(
                hyper::header::CONTENT_TYPE,
                hyper::http::HeaderValue::from_str("application/json").unwrap(), // TODO: Remove unwrap
            );
            Ok(response)
        }
        _ => todo!(),
    }
}
