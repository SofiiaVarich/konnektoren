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
