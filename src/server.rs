use axum::http::{uri::Uri, Request, Response};
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashSet;
use std::net::SocketAddr;

type Client = hyper::client::Client<HttpsConnector<HttpConnector>, Body>;

#[derive(Deserialize)]
struct User {
    login: String,
}

#[derive(Deserialize)]
struct PullRequest {
    user: User,
}

#[derive(Deserialize)]
struct Webhook {
    pull_request: PullRequest,
}

lazy_static! {
    static ref BANNED_SET: HashSet<String> = {
        let mut set: HashSet<String> = HashSet::new();
        set.insert("dependabot[bot]".to_string());
        set
    };
}

async fn webhook_handler(State(client): State<Client>, mut req: Request<Body>) -> Response<Body> {
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or_else(|| req.uri().path());

    let uri = format!("https://discord.com{path_query}");

    req.headers_mut().remove("host");
    *req.uri_mut() = Uri::try_from(uri).unwrap();

    let bytes = hyper::body::to_bytes(req.body_mut()).await.unwrap();

    if let Ok(webhook) = serde_json::from_slice::<Webhook>(&bytes) {
        if BANNED_SET.contains(&webhook.pull_request.user.login) {
            return Response::builder().body(Body::empty()).unwrap();
        }
    }

    *req.body_mut() = Body::from(bytes);

    client.request(req).await.unwrap()
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

pub async fn start_server() {
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/webhooks/*etc", post(webhook_handler))
        .with_state(client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
