use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DidCommInvitation {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
    label: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
    #[serde(rename = "recipientKeys")]
    recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    routing_keys: Vec<String>,
    did: String,
}

impl DidCommInvitation {
    pub fn new(id: String, base_url: String) -> Self {
        let service_endpoint = format!("{}/didcomm", base_url);
        let type_ = "https://didcomm.org/connections/1.0/invitation".to_string();
        let label = "Invitation to connect".to_string();
        let recipient_keys = vec!["".to_string()];
        let routing_keys = vec!["".to_string()];
        let did = "did:example:123456789abcdefghi".to_string();

        DidCommInvitation {
            type_,
            id,
            label,
            service_endpoint,
            recipient_keys,
            routing_keys,
            did,
        }
    }
}
