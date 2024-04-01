use std::env;

fn main() {
    // Fetch the private key from an environment variable at build time
    let private_key =
        env::var("SIGNATURE_PRIVATE_KEY").unwrap_or_else(|_| "default_private_key".to_string());

    println!("cargo:rustc-env=SIGNATURE_PRIVATE_KEY={}", private_key);
}
