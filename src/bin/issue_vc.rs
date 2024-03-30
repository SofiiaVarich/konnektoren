use axum::{
    extract::State,
    extract::{Extension, Path, Query},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use base64::engine::general_purpose;
use base64::Engine as _;
use dotenv::dotenv;
use konnektoren_worker_rust::vc::DidCommInvitation;
use ngrok::prelude::*;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{net::SocketAddr, sync::Arc};
use tokio::task;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use urlencoding::encode;

#[derive(Clone)]
struct AppState {
    base_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut tun = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await
        .unwrap()
        .http_endpoint()
        .listen()
        .await
        .unwrap();

    let base_url = tun.url().to_string();

    tracing::info!("App URL: {:?}", base_url);

    let ngrok_tunnel = task::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tun.forward_tcp(addr).await.unwrap();
    });

    let base_url = base_url.clone();

    let shared_state = AppState { base_url };

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello from ngrok-rust!" }))
        .route(
            "/didcomm",
            post(|| async {
                println!("Hello from didcomm!");
                "Hello from didcomm!"
            }),
        )
        .route("/ssi", get(|| async { "Hello from ssi!" }))
        .route("/oob", get(oob_invitation))
        .route("/qr/:id", get(invitation_qr_code))
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let server = task::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    let _ = tokio::try_join!(ngrok_tunnel, server)?;

    Ok(())
}

#[derive(Deserialize)]
struct IdQuery {
    id: String,
}

async fn oob_invitation(
    Query(query): Query<IdQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let base64_oob = generate_base64_oob(&query.id, (*state.base_url).to_string());
    let redirect_url = format!("/ssi?oob={}&id={}", encode(&base64_oob), query.id);
    Redirect::temporary(&redirect_url)
}

fn generate_base64_oob(id: &str, base_url: String) -> String {
    let oob_data =
        serde_json::to_string(&DidCommInvitation::new(id.to_string(), base_url)).unwrap();
    general_purpose::STANDARD.encode(&oob_data)
}

async fn invitation_qr_code(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let url = format!("{}/oob?id={}", (*state.base_url).to_string(), id);
    let code = QrCode::new(&url).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build();
    format!(
        "Hello from invitation_qr_code! id: {}\n{}\n{}",
        id, string, url
    )
}

async fn issue_qr_code(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let didcomm_inv = DidCommInvitation::new(id.to_string(), (*state.base_url).to_string());
    let code = QrCode::new(serde_json::to_string(&didcomm_inv).unwrap()).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build();
    string
}
