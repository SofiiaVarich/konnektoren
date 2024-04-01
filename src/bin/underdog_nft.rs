use bip39::Mnemonic;
use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use solana_sdk::{
    derivation_path::DerivationPath,
    signature::{Keypair, Signer},
    signer::SeedDerivable,
};
use std::env;

const UNDERDOG_API_ENDPOINT: &str = "https://dev.underdogprotocol.com";

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

    let req = client
        .post(format!(
            "{UNDERDOG_API_ENDPOINT}/v2/projects/{project_id}/nfts"
        ))
        .header("Authorization", format!("Bearer {}", UNDERDOG_API_KEY))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json!({
            "name": "Konnektoren",
            "symbol": "KHN",
            "image": "https://konnektoren.help/favicon.png",
            "receiverAddress": payer.pubkey().to_string(),
        }))?);

    let res = req.send().await?;

    let body: Value = res.json().await?;
    println!("Response: {}", body);

    let nft_id = body["nftId"].as_u64().unwrap();

    let mut pending = true;

    while pending {
        let res_nft = client
            .get(format!(
                "{UNDERDOG_API_ENDPOINT}/v2/projects/{project_id}/nfts/{nft_id}"
            ))
            .header("Authorization", format!("Bearer {}", UNDERDOG_API_KEY))
            .send()
            .await?;

        let body_nft = res_nft.text().await?;
        println!("Response: {}", body_nft);

        let nft: Value = serde_json::from_str(&body_nft)?;

        if nft["status"].as_str().unwrap() == "pending" {
            println!("NFT is pending...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        } else {
            pending = false;
        }
    }

    let res_nft = client
        .get(format!(
            "{UNDERDOG_API_ENDPOINT}/v2/projects/{project_id}/nfts/{nft_id}"
        ))
        .header("Authorization", format!("Bearer {}", UNDERDOG_API_KEY))
        .send()
        .await?;

    let body_nft: Value = res_nft.json().await?;
    println!("Response: {}", body_nft);

    let mint_address = body_nft["mintAddress"].as_str().unwrap();

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
