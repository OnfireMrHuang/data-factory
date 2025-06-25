mod routes;
mod models;
mod services;
mod utils;
mod repositories;

#[tokio::main]
async fn main() {
    // initialize configuration
    utils::config::Setting::init();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = routes::router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
