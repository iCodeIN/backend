#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/auth/token")
        .route("/auth/onboard")
        .route("/auth/changepass")
        .route("/msg")
        .route("/sock")
        .with_state();
}


