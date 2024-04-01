use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub const UNDERDOG_API_ENDPOINT: &str = "https://dev.underdogprotocol.com";

pub async fn create_nft(receiver: String, project_id: String, api_key: &String) -> Result<Value> {
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
            "image": "https://konnektoren.help/favicon.png",
            "receiverAddress": receiver,
        }))?);

    let res = req.send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    Ok(json)
}

pub async fn fetch_mint_address(
    project_id: String,
    nft_id: String,
    api_key: &String,
) -> Result<String> {
    let client = Client::new();
    let mut pending = true;
    let mut timeout = 10;
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

        if pending {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    match nft.get("mintAddress") {
        Some(mint_address) => Ok(mint_address.as_str().unwrap().to_string()),
        None => Err(anyhow::anyhow!("Mint address not found")),
    }
}
