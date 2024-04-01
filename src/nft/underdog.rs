use anyhow::Result;
use konnektoren::model::TestType;
use reqwest::Client;
use serde_json::{json, Value};

pub const UNDERDOG_API_ENDPOINT: &str = "https://dev.underdogprotocol.com";

fn nft_img_url(test_type: &TestType) -> String {
    match test_type {
        TestType::Konnektoren => "/assets/nft-konnektoren.png".to_string(),
        TestType::Adjectives => "/assets/nft-adjectives.png".to_string(),
        TestType::Verbs => "/assets/nft-verbs.png".to_string(),
        TestType::Nomen => "/assets/nft-nouns.png".to_string(),
    }
}

pub async fn create_nft(
    test_type: &TestType,
    receiver: &String,
    project_id: &String,
    api_key: &String,
) -> Result<Value> {
    let nft_img_url = nft_img_url(test_type);

    let client = Client::new();
    let req = client
        .post(format!(
            "{UNDERDOG_API_ENDPOINT}/v2/projects/{project_id}/nfts"
        ))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&json!({
            "name": "Konnektoren",
            "symbol": "KHN",
            "image": format!("https://konnektoren.help{nft_img_url}"),
            "receiverAddress": receiver,
        }))?);

    let res = req.send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    Ok(json)
}

pub async fn fetch_mint_address(
    project_id: &String,
    nft_id: &String,
    api_key: &String,
) -> Result<String> {
    let client = Client::new();
    let mut pending = true;
    let mut timeout = 100;
    let mut nft: Value = json!({});

    while timeout > 0 && pending {
        let res = client
            .get(format!(
                "{UNDERDOG_API_ENDPOINT}/v2/projects/{project_id}/nfts/{nft_id}"
            ))
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;
        let body_nft = res.text().await?;

        nft = serde_json::from_str(&body_nft)?;

        log::debug!("NFT: {:?}", nft);

        match nft.get("status") {
            Some(status) => {
                if status.as_str().unwrap() == "pending" {
                    pending = true;
                } else {
                    pending = false;
                }
            }
            None => {
                pending = true;
            }
        }

        timeout -= 1;
    }

    let mint_address = match nft.get("mintAddress") {
        Some(mint_address) => mint_address.as_str().unwrap().to_string(),
        None => return Err(anyhow::anyhow!("Mint address not found")),
    };
    if mint_address.is_empty() {
        return Err(anyhow::anyhow!("Mint address not found"));
    }

    Ok(mint_address)
}
