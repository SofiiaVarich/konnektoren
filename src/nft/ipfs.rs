use anyhow::{anyhow, Result};
use base64;
use reqwest::{multipart, Client};

use serde_json::Value;

pub struct Ipfs {
    image: Vec<u8>,
    api_key: String,
}

impl Ipfs {
    pub fn new(image: Vec<u8>, api_key: String) -> Self {
        Ipfs { image, api_key }
    }

    pub async fn upload(&self) -> Result<String> {
        let client = Client::new();

        // Prepare the multipart/form-data request with the image
        let form = multipart::Form::new().part(
            "file",
            multipart::Part::bytes(self.image.clone()).file_name("image.png"),
        );

        // Send the request to NFT.storage
        let response = client
            .post("https://api.nft.storage/upload")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        // Handle the response
        if response.status().is_success() {
            let response_json: Value = response
                .json()
                .await
                .map_err(|e| anyhow!("Failed to parse JSON response: {}", e))?;

            // Extract the CID from the response
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
