use bip39::Mnemonic;
use dotenv::dotenv;
use mpl_core::instructions::CreateV1Builder;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    derivation_path::DerivationPath,
    signature::{Keypair, Signer},
    signer::SeedDerivable,
    transaction::Transaction,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let mnemonic_str = env::var("MNEMONIC").expect("MNEMONIC environment variable not found");
    let mnemonic_passphrase = env::var("MNEMONIC_PASSPHRASE").unwrap_or_default();

    println!("Mnemonic loaded from environment variable {}", mnemonic_str);

    let payer = mnemonic_to_keypair(&mnemonic_str, &mnemonic_passphrase)?;

    println!("Account {}", payer.pubkey().to_string());

    let balance = client.get_balance(&payer.pubkey())?;
    println!("Payer balance: {}", balance);

    let metadata_uri = "https://konnektoren.help/nft-metadata.json";

    match create_asset(&client, &payer, metadata_uri) {
        Ok(_) => println!("Metadata account created successfully."),
        Err(e) => eprintln!("Failed to create metadata account: {}", e),
    }

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

pub fn create_asset(
    client: &RpcClient,
    payer: &Keypair,
    metadata_uri: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let asset = Keypair::new();

    let create_asset_ix = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(payer.pubkey())
        .name("Konnektoren Help".into())
        .uri(metadata_uri.into())
        .instruction();

    let signers = vec![&asset, &payer];

    let last_blockhash = client.get_latest_blockhash()?;

    let create_asset_tx = Transaction::new_signed_with_payer(
        &[create_asset_ix],
        Some(&payer.pubkey()),
        &signers,
        last_blockhash,
    );

    let res = client.send_and_confirm_transaction(&create_asset_tx)?;

    println!("Signature: {:?}", res);
    Ok(())
}
