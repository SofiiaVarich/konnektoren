use bip39::Mnemonic;
use dotenv::dotenv;
use mpl_core::{
    instructions::{CreateCollectionV1Builder, CreateV1Builder},
    Asset,
};
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

    let collection = Keypair::new();

    let asset = Keypair::new();

    match create_asset_in_collection(&client, &payer, &asset, &collection, metadata_uri) {
        Ok(_) => println!("Collection account created successfully."),
        Err(e) => eprintln!("Failed to create collection account: {}", e),
    }

    println!("Collection account {}", collection.pubkey().to_string());

    /*
    match create_asset(&client, &payer, &asset, metadata_uri) {
        Ok(_) => println!("Metadata account created successfully."),
        Err(e) => eprintln!("Failed to create metadata account: {}", e),
    }
    */

    println!("Asset account {}", asset.pubkey().to_string());

    let asset_id = asset.pubkey();

    let rpc_data = client.get_account_data(&asset_id)?;

    let asset = Asset::from_bytes(&rpc_data).unwrap();

    println!("{:?}", asset);

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
    asset: &Keypair,
    metadata_uri: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_asset_in_collection(
    client: &RpcClient,
    payer: &Keypair,
    asset: &Keypair,
    collection: &Keypair,
    metadata_uri: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let create_collection_ix = CreateCollectionV1Builder::new()
        .collection(collection.pubkey())
        .payer(payer.pubkey())
        .name("Konnektoren Help".into())
        .uri(metadata_uri.into())
        .instruction();

    let signers = vec![&collection, &payer];

    let last_blockhash = client.get_latest_blockhash()?;

    let create_collection_tx = Transaction::new_signed_with_payer(
        &[create_collection_ix],
        Some(&payer.pubkey()),
        &signers,
        last_blockhash,
    );

    let res = client.send_and_confirm_transaction(&create_collection_tx)?;

    println!("Signature: {:?}", res);

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
