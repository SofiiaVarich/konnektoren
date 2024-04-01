use worker::*;

mod certificate;
pub mod nft;
mod routes;
pub mod vc;

mod services;
#[cfg(feature = "ipfs")]
use routes::handle_ipfs_request;

use routes::{
    handle_certificate_request, handle_leaderboard_request, handle_metadata_request,
    handle_mint_request,
};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    #[cfg(feature = "debug")]
    worker_logger::init_with_level(&log::Level::Debug);

    let router = Router::new()
        .get_async("/certificate/:code", handle_certificate_request)
        .get_async("/metadata/:code", handle_metadata_request);

    let router = router.get_async("/leaderboard.json", handle_leaderboard_request);

    #[cfg(feature = "ipfs")]
    let router = router.get_async("/generate/:code", handle_generate_request);

    let router = router.get_async("/mint", handle_mint_request);

    router.run(req, env).await
}
