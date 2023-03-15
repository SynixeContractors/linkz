use std::net::SocketAddr;

use hyper::{
    header, server::conn::http1, service::service_fn, Body, Method, Request, Response, StatusCode,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(redirect))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn redirect(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap());
    }
    let path = req.uri().path();
    let target = match path {
        "/discord" => Some("https://discord.gg/SZhzv6z"),
        "/donate" => Some("https://www.patreon.com/synixe"),
        "/twitter" => Some("https://twitter.com/SynixePMC"),
        "/github" => Some("https://github.com/SynixeContractors"),
        "/docs" => Some("https://docs.synixe.contractors"),
        "" => Some("https://synixe.contractors"),
        _ => None,
    };
    if let Some(target) = target {
        return Ok(Response::builder()
            .status(StatusCode::FOUND)
            .header(header::LOCATION, target)
            .body(Body::empty())
            .unwrap());
    }
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, complex(path))
        .body(Body::empty())
        .unwrap())
}

fn complex(link: &str) -> String {
    if link.starts_with("/certs/") {
        let link = link.strip_prefix("/certs/").unwrap();
        format!("https://docs.synixe.contractors/#/certs/{}", link)
            .parse()
            .unwrap()
    } else {
        "https://synixe.contractors".to_string()
    }
}
