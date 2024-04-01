use bip39::Mnemonic;
use dotenv::dotenv;
use konnektoren_worker_rust::nft::{create_nft, fetch_mint_address};
use reqwest::Client;
use serde_json::{json, Value};
use solana_sdk::{
    derivation_path::DerivationPath,
    signature::{Keypair, Signer},
    signer::SeedDerivable,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let UNDERDOG_API_KEY =
        env::var("UNDERDOG_API_KEY").expect("UNDERDOG_API_KEY environment variable not found");

    let client = Client::new();

    let mnemonic_str = env::var("MNEMONIC").expect("MNEMONIC environment variable not found");
    let mnemonic_passphrase = env::var("MNEMONIC_PASSPHRASE").unwrap_or_default();

    let project_id = env::var("UNDERDOG_PROJECT_ID")
        .expect("UNDERDOG_PROJECT_ID environment variable not found");

    let payer = mnemonic_to_keypair(&mnemonic_str, &mnemonic_passphrase)?;

    println!("Payer: {}", payer.pubkey().to_string());

    let body: Value =
        create_nft(&payer.pubkey().to_string(), &project_id, &UNDERDOG_API_KEY).await?;

    println!("Response: {}", body);

    let nft_id = format!("{}", body["nftId"].as_i64().unwrap());

    let mint_address = fetch_mint_address(&project_id, &nft_id, &UNDERDOG_API_KEY).await?;

    println!("https://xray.helius.xyz/token/{mint_address}?network=devnet");

    Ok(())
}

fn mnemonic_to_keypair(
    mnemonic_str: &str,
    _mnemonic_passphrase: &str,
) -> Result<Keypair, Box<dyn std::error::Error>> {
    let mnemonic = Mnemonic::parse(mnemonic_str)?;
    let seed = mnemonic.to_seed("");
    let keypair = Keypair::from_seed_and_derivation_path(
        &seed,
        Some(DerivationPath::new_bip44(Some(0), Some(0))),
    )?;

    Ok(keypair)
}
