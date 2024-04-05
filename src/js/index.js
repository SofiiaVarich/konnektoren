async function connectSolanaWallet() {
  try {
    const solana = window.solana;
    if (solana) {
      const response = await solana.connect();
      return response.publicKey.toString();
    } else {
      console.error("Solana object not found.");
      return null;
    }
  } catch (err) {
    console.error("Failed to connect to Solana wallet:", err);
    return null;
  }
}
