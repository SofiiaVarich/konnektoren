# Konnektoren Server

## Develop

```sh
npx wrangler dev
```

## Deploy

```sh
npx wrangler deploy
```

## Setup

Create a `.dev.vars` file with the following content:

```sh
IPFS_API_KEY="..." # NFT Storage API
```

Update the secrets:

```sh
npx wrangler secret put IPFS_API_KEY
```

## Deploy NFT

Create a .env file with the following content:

```env
MNEMONIC="hello world ..."
```

Run the following command:

```sh
cargo run --features=deploy-nft --bin deploy-nft
```

## Run VC Issue service

```sh
cargo run --features=issue-vc --bin issue-vc
```

## Run create NFT with underdog

```sh
cargo run --features=underdog --bin underdog-nft
```
