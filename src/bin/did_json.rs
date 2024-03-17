use konnektoren::utils::keypair_from_static_str;

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct PublicKey {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    controller: String,
    #[serde(rename = "publicKeyHex")]
    public_key_hex: String,
}

#[derive(Serialize, Deserialize)]
struct Service {
    id: String,
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
}

#[derive(Serialize, Deserialize)]
struct DIDDocument {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    #[serde(rename = "publicKey")]
    public_key: Vec<PublicKey>,
    service: Vec<Service>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain = args.get(1).unwrap_or(&"example.com".to_string()).clone();

    let (_signing_key, verify_key) = keypair_from_static_str();

    let did_document = DIDDocument {
        context: "https://www.w3.org/ns/did/v1".to_string(),
        id: format!("did:web:{}", domain),
        public_key: vec![PublicKey {
            id: format!("did:web:{}#owner", domain),
            key_type: "Ed25519VerificationKey2018".to_string(),
            controller: format!("did:web:{}", domain),
            public_key_hex: hex::encode(verify_key.as_bytes()),
        }],
        service: vec![Service {
            id: format!("did:web:{}#vcs", domain),
            service_type: "VerifiableCredentialService".to_string(),
            service_endpoint: format!("https://{}/vc/", domain),
        }],
    };

    let did_json = to_string_pretty(&did_document).unwrap();

    let mut file = File::create("did.json").unwrap();
    file.write_all(did_json.as_bytes()).unwrap();

    println!("DID document generated successfully for domain: {}", domain);
}
