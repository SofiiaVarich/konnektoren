use anyhow::{anyhow, Result};
use reqwest::{multipart, Client};

use serde_json::Value;

pub struct Ipfs {
    api_key: String,
}

impl Ipfs {
    pub fn new(api_key: String) -> Self {
        Ipfs { api_key }
    }

    pub async fn upload(&self, data: Vec<u8>, filename: String) -> Result<String> {
        let client = Client::new();

        let form = multipart::Form::new().part(
            "file",
            multipart::Part::bytes(data.clone()).file_name(filename),
        );

        let response = client
            .post("https://api.nft.storage/upload")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        if response.status().is_success() {
            let response_json: Value = response
                .json()
                .await
                .map_err(|e| anyhow!("Failed to parse JSON response: {}", e))?;

            if let Some(value) = response_json.get("value") {
                if let Some(cid) = value.get("cid").and_then(|c| c.as_str()) {
                    return Ok(cid.to_string());
                }
            }
            Err(anyhow!("CID not found in response"))
        } else {
            Err(anyhow!(
                "Failed to upload to NFT.storage: Status {}",
                response.status()
            ))
        }
    }
}
