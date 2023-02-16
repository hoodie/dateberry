use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = dateberry::setup_router();

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
