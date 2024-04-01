use worker::*;

mod certificate;
pub mod nft;
mod routes;
pub mod vc;

mod services;
use routes::{
    handle_certificate_request, handle_generate_request, handle_leaderboard_request,
    handle_metadata_request, handle_mint_request,
};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    worker_logger::init_with_level(&log::Level::Debug);

    let router = Router::new();
    router
        .get_async("/certificate/:code", handle_certificate_request)
        .get_async("/metadata/:code", handle_metadata_request)
        .get_async("/generate/:code", handle_generate_request)
        .get_async("/leaderboard.json", handle_leaderboard_request)
        .get_async("/mint", handle_mint_request)
        .run(req, env)
        .await
}
