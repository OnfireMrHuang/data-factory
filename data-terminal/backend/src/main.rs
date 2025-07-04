mod routes;
mod models;
mod services;
mod utils;
mod repositories;
mod autofac;

#[tokio::main]
async fn main() {
    // initialize configuration
    utils::config::Setting::init();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize database
    utils::database::config_db_init().await;

    // build our application with a route
    let app = routes::router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
