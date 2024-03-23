use anyhow::{anyhow, Result};
use image::ImageOutputFormat;
use konnektoren::model::TestResult;
use konnektoren::utils::create_certificate;
use std::io::Cursor;
use urlencoding::decode;

pub struct Certificate {
    pub issuer: String,
    pub test_result: Option<TestResult>,
    pub url: String,
}

impl Default for Certificate {
    fn default() -> Self {
        Self {
            issuer: "issuer".into(),
            test_result: None,
            url: "url".into(),
        }
    }
}

impl Certificate {
    pub fn new(issuer: String, test_result: TestResult, url: String) -> Self {
        Self {
            issuer,
            test_result: Some(test_result),
            url,
        }
    }

    pub fn from_base64(data: &str) -> Result<Self> {
        let base64: String = decode(data)?.to_string();
        let test_result = Some(TestResult::from_base64(&base64)?);
        Ok(Self {
            test_result,
            ..Default::default()
        })
    }

    pub fn to_png(&self) -> Result<Vec<u8>> {
        match create_certificate(self.test_result.as_ref().unwrap(), &self.url, &self.issuer) {
            Ok(image) => {
                let mut bytes: Vec<u8> = Vec::new();
                image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)?;
                Ok(bytes)
            }
            Err(_) => Err(anyhow!("Failed to create certificate image as png.")),
        }
    }
}
