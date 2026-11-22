use backend::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
