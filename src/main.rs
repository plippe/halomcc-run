mod campaign_modes;
mod chainable;
mod difficulties;
mod error;
mod games;
mod graphql;
mod halo_waypoint;
mod missions;
mod service_records;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use std::net::SocketAddr;

use crate::graphql::service::Service;

fn addr() -> SocketAddr {
    let ip = [0, 0, 0, 0];
    let port = std::env::var("PORT")
        .map(|it| it.parse().unwrap())
        .unwrap_or(3000);

    (ip, port).into()
}

#[derive(Clone)]
struct NotFound;
impl NotFound {
    async fn call(&self, _req: Request<Body>) -> Result<Response<Body>, hyper::error::Error> {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())
    }

    fn default() -> Self {
        Self
    }
}

#[tokio::main]
async fn main() {
    let graphql_service = Service::default();
    let not_found_service = NotFound::default();

    let new_service = make_service_fn(move |_| {
        let graphql_service = graphql_service.clone();
        let not_found_service = not_found_service.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let graphql_service = graphql_service.clone();
                let not_found_service = not_found_service.clone();

                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => graphql_service.graphiql(req).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            graphql_service.graphql(req).await
                        }
                        _ => not_found_service.call(req).await,
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr()).serve(new_service);
    println!("Listening on http://{}", server.local_addr());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
